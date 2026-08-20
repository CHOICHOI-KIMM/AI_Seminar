import { useState, useEffect, useRef } from 'react';

interface Props {
  label: string;
  value: number;
  unit: string;
  onChange: (v: number) => void;
  step?: number;
  min?: number;
  max?: number;
}

export default function FieldGroup({ label, value, unit, onChange, step = 1, min, max }: Props) {
  const [text, setText] = useState(String(value));
  const inputRef = useRef<HTMLInputElement>(null);

  useEffect(() => {
    // Only sync from parent when not focused, to avoid disrupting user input
    if (document.activeElement !== inputRef.current) {
      setText(String(value));
    }
  }, [value]);

  const handleChange = (e: React.ChangeEvent<HTMLInputElement>) => {
    const val = e.target.value;
    setText(val);
    // Eagerly update parent state for valid numbers
    // (eliminates blur → click race condition with Solve button)
    const n = e.target.valueAsNumber;
    if (!isNaN(n) && isFinite(n)) {
      onChange(n);
    }
  };

  const commit = () => {
    const n = parseFloat(text);
    if (!isNaN(n)) {
      onChange(n);
    } else {
      setText(String(value));
    }
  };

  return (
    <div className="flex items-center justify-between gap-2 py-0.5">
      <label className="text-[13px] text-text-muted whitespace-nowrap min-w-0 truncate flex-1">
        {label}
      </label>
      <div className="flex items-center gap-1">
        <input
          ref={inputRef}
          type="number"
          value={text}
          step={step}
          min={min}
          max={max}
          onChange={handleChange}
          onBlur={commit}
          onKeyDown={e => e.key === 'Enter' && commit()}
          className="w-20 px-1.5 py-0.5 text-[13px] text-right bg-white border border-slate-200 rounded font-mono tabular-nums
                     focus:outline-none focus:border-accent focus:ring-1 focus:ring-accent/30
                     text-text-dark"
        />
        {unit && <span className="text-xs text-text-muted w-8">{unit}</span>}
      </div>
    </div>
  );
}
