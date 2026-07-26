"""
단독 HTML 대시보드 빌드 — 타 컴퓨터·오프라인에서 그냥 열리는 완전한 문서
=====================================================================
입력: dashboard_body.html(본문 템플릿, /*__DATA__*/ 자리) + dashboard_data.json
출력: DLC별해석_대시보드.html (DOCTYPE·head·리셋·본문 일체 · 외부 의존 0)
갱신: build_dashboard_data.py 재실행 → 본 스크립트 재실행이면 최신 데이터 반영.
"""
import json
import os

HERE = os.path.dirname(os.path.abspath(__file__))
BODY = os.path.join(HERE, "dashboard_body.html")
DATA = os.path.join(HERE, "dashboard_data.json")
OUT = os.path.join(HERE, "DLC별해석_대시보드.html")

HEAD = """<!DOCTYPE html>
<html lang="ko">
<head>
<meta charset="utf-8">
<meta name="viewport" content="width=device-width, initial-scale=1">
<meta name="color-scheme" content="light dark">
<title>DLC별 피로해석 — 방법론·결과 대시보드</title>
<style>
  html,body{margin:0;padding:0}
  *,*::before,*::after{box-sizing:border-box}
  img,svg{max-width:100%}
  button{font:inherit}
</style>
"""


def main():
    body = open(BODY, encoding="utf-8").read()
    data = open(DATA, encoding="utf-8").read().strip()
    if "/*__DATA__*/{}" not in body:
        raise SystemExit("본문 템플릿에 /*__DATA__*/{} 자리표시자가 없습니다.")
    body = body.replace("/*__DATA__*/{}", data, 1)
    html = HEAD + "<body>\n" + body + "\n</body>\n</html>\n"
    open(OUT, "w", encoding="utf-8").write(html)
    kb = len(html.encode("utf-8")) / 1024
    gen = json.loads(data)["generated"]
    ndone = json.loads(data)["kpi"]["n_done"]
    print(f"[저장] {OUT}")
    print(f"  {kb:.0f} KB · 자체완결(외부 의존 0) · 기준 {gen} · {ndone}/111 DLC")
    print("  더블클릭으로 브라우저에서 바로 열림 (오프라인 가능)")


if __name__ == "__main__":
    main()
