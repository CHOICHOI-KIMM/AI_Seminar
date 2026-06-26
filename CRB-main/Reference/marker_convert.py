"""
PDF를 고품질 마크다운으로 변환하는 스크립트 (marker 라이브러리 사용)

marker는 표, 수식, 이미지를 정확하게 추출하며 OCR 기능도 지원합니다.
모델을 1회만 로딩하여 반복 변환 시 성능을 최적화합니다.

사용법:
    python marker_convert.py <PDF파일> [출력파일] [옵션]
    python marker_convert.py --dir <폴더> [출력폴더] [옵션]
    python marker_convert.py --server [--port 8000]

옵션:
    --no-images         이미지 추출 생략 (속도 향상)
    --pages 0,5-10      특정 페이지만 변환
    --compile           torch.compile 적용 (초기 느리나 반복 시 빠름)

예시:
    python marker_convert.py document.pdf
    python marker_convert.py document.pdf output.md --no-images
    python marker_convert.py --dir pdfs/ markdown/
    python marker_convert.py --server --port 8000
"""

import sys
import io
import os
import time
import argparse
from pathlib import Path

# Windows 콘솔 UTF-8 출력 설정
sys.stdout = io.TextIOWrapper(sys.stdout.buffer, encoding='utf-8', errors='replace')
sys.stderr = io.TextIOWrapper(sys.stderr.buffer, encoding='utf-8', errors='replace')

# 모듈 레벨 캐시: 프로세스 내에서 모델 1회만 로드
_model_dict = None
_converter = None


def _log(msg: str):
    print(msg, file=sys.stderr)


def _get_models():
    """모델을 캐싱하여 1회만 로드합니다."""
    global _model_dict
    if _model_dict is None:
        from marker.models import create_model_dict
        t0 = time.perf_counter()
        _log("모델 로딩 중...")
        _model_dict = create_model_dict()
        _log(f"모델 로딩 완료 ({time.perf_counter() - t0:.1f}초)")
    return _model_dict


def _get_converter(config_override: dict | None = None):
    """PdfConverter를 캐싱하여 재사용합니다."""
    global _converter
    if _converter is None:
        from marker.converters.pdf import PdfConverter
        kwargs = {"artifact_dict": _get_models()}
        if config_override:
            from marker.config.parser import ConfigParser
            cfg = ConfigParser(config_override)
            kwargs["config"] = cfg
        _converter = PdfConverter(**kwargs)
    return _converter


def convert_pdf(pdf_path: str, disable_images: bool = False,
                page_range: str | None = None) -> tuple[str, dict]:
    """
    PDF를 마크다운으로 변환합니다. 모델은 캐싱되어 재사용됩니다.

    Args:
        pdf_path: PDF 파일 경로
        disable_images: True면 이미지 추출 생략 (속도 향상)
        page_range: 변환할 페이지 범위 (예: "0,5-10")

    Returns:
        (markdown_text, images_dict) 튜플
    """
    from marker.output import text_from_rendered

    config = {}
    if disable_images:
        config["disable_image_extraction"] = True
    if page_range:
        config["page_range"] = page_range

    converter = _get_converter(config if config else None)

    t0 = time.perf_counter()
    _log(f"변환 중: {pdf_path}")
    rendered = converter(pdf_path)
    markdown_text, _, images = text_from_rendered(rendered)
    _log(f"변환 완료 ({time.perf_counter() - t0:.1f}초)")

    return markdown_text, images


def convert_and_save(pdf_path: str, output_path: str | None = None,
                     disable_images: bool = False,
                     page_range: str | None = None) -> str:
    """PDF를 마크다운으로 변환하고 파일로 저장합니다."""
    pdf_file = Path(pdf_path)

    if output_path is None:
        output_path = str(pdf_file.with_suffix('.md'))

    output_file = Path(output_path)
    images_dir = output_file.parent / f"{output_file.stem}_images"

    markdown_text, images = convert_pdf(pdf_path, disable_images, page_range)

    output_file.write_text(markdown_text, encoding='utf-8')

    if images:
        images_dir.mkdir(parents=True, exist_ok=True)
        for img_name, img_data in images.items():
            img_path = images_dir / img_name
            if hasattr(img_data, 'save'):
                img_data.save(str(img_path))
            else:
                img_path.write_bytes(img_data)
        _log(f"이미지 {len(images)}개 저장: {images_dir}")

    return markdown_text


def convert_directory(input_dir: str, output_dir: str | None = None,
                      disable_images: bool = False,
                      page_range: str | None = None) -> dict[str, str]:
    """디렉토리 내 모든 PDF를 변환합니다. 모델은 1회만 로드됩니다."""
    results = {}
    input_path = Path(input_dir)

    if output_dir:
        output_path = Path(output_dir)
        output_path.mkdir(parents=True, exist_ok=True)
    else:
        output_path = input_path

    pdf_files = sorted(set(
        list(input_path.glob('*.pdf')) + list(input_path.glob('*.PDF'))
    ))

    if not pdf_files:
        _log(f"PDF 파일을 찾을 수 없습니다: {input_dir}")
        return results

    _log(f"총 {len(pdf_files)}개 PDF 발견")

    # 모델 사전 로드 (첫 파일 변환 전 1회)
    _get_models()

    for i, pdf_file in enumerate(pdf_files, 1):
        try:
            out_file = output_path / f"{pdf_file.stem}.md"
            _log(f"[{i}/{len(pdf_files)}] {pdf_file.name}")
            markdown = convert_and_save(
                str(pdf_file), str(out_file), disable_images, page_range
            )
            results[pdf_file.name] = markdown
            print(f"✓ {pdf_file.name} → {out_file.name}")
        except Exception as e:
            print(f"✗ {pdf_file.name} - {e}")

    return results


def run_server(port: int = 8000, host: str = "127.0.0.1"):
    """marker_server를 실행합니다 (모델 상주, 반복 변환에 최적)."""
    try:
        from marker.server import app
        import uvicorn
        _log(f"서버 시작: http://{host}:{port}")
        _log("POST /marker       - filepath로 변환")
        _log("POST /marker/upload - 파일 업로드 변환")
        uvicorn.run(app, host=host, port=port)
    except ImportError:
        _log("서버 모드에는 uvicorn이 필요합니다: pip install uvicorn")
        sys.exit(1)


def setup_env(compile_models: bool = False):
    """성능 최적화 환경변수를 설정합니다."""
    # GPU 자동 감지 (이미 설정되지 않은 경우)
    if "TORCH_DEVICE" not in os.environ:
        try:
            import torch
            if torch.cuda.is_available():
                os.environ["TORCH_DEVICE"] = "cuda"
                _log("GPU 감지: CUDA 사용")
            elif hasattr(torch.backends, "mps") and torch.backends.mps.is_available():
                os.environ["TORCH_DEVICE"] = "mps"
                _log("GPU 감지: MPS 사용")
            else:
                _log("GPU 미감지: CPU 사용")
        except ImportError:
            pass

    if compile_models:
        os.environ["COMPILE_ALL"] = "true"
        _log("torch.compile 활성화")


def main():
    parser = argparse.ArgumentParser(
        description="PDF → Markdown 변환 (marker)",
        formatter_class=argparse.RawDescriptionHelpFormatter,
        epilog=__doc__
    )
    parser.add_argument("input", nargs="?", help="PDF 파일 또는 --dir 시 입력 폴더")
    parser.add_argument("output", nargs="?", help="출력 파일/폴더 경로")
    parser.add_argument("--dir", action="store_true", help="폴더 일괄 변환 모드")
    parser.add_argument("--server", action="store_true", help="서버 모드 (모델 상주)")
    parser.add_argument("--port", type=int, default=8000, help="서버 포트 (기본: 8000)")
    parser.add_argument("--no-images", action="store_true", help="이미지 추출 생략")
    parser.add_argument("--pages", type=str, default=None, help="페이지 범위 (예: 0,5-10)")
    parser.add_argument("--compile", action="store_true", help="torch.compile 적용")

    args = parser.parse_args()

    # 서버 모드
    if args.server:
        setup_env(args.compile)
        run_server(port=args.port)
        return

    if not args.input:
        parser.print_help()
        sys.exit(0)

    setup_env(args.compile)

    # 폴더 일괄 변환
    if args.dir:
        convert_directory(args.input, args.output, args.no_images, args.pages)
        return

    # 단일 파일 변환
    if not Path(args.input).exists():
        _log(f"✗ 파일을 찾을 수 없습니다: {args.input}")
        sys.exit(1)

    try:
        if args.output:
            convert_and_save(args.input, args.output, args.no_images, args.pages)
            print(f"✓ 저장 완료: {args.output}")
        else:
            markdown, _ = convert_pdf(args.input, args.no_images, args.pages)
            print(markdown)
    except Exception as e:
        _log(f"✗ 변환 실패: {e}")
        sys.exit(1)


if __name__ == "__main__":
    main()
