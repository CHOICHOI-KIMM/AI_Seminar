import { useAppState } from '../../store';

export default function AlertPanel() {
  const { state } = useAppState();
  const { result, error } = state;

  const alerts = result?.alerts ?? [];

  if (!error && alerts.length === 0) return null;

  return (
    <div className="border-t border-panel-border p-3">
      <h3 className="text-sm font-semibold text-text-dark uppercase tracking-wider mb-2">Alerts</h3>

      {error && (
        <div className="flex items-start gap-2 p-2 rounded bg-red-50 border border-red-200 mb-2">
          <span className="text-danger text-sm mt-0.5">!</span>
          <p className="text-[13px] text-red-700 break-words">{error}</p>
        </div>
      )}

      {alerts.map((alert, i) => {
        const colors = {
          Info: 'bg-blue-50 border-blue-200 text-blue-700',
          Warning: 'bg-amber-50 border-amber-200 text-amber-700',
          Critical: 'bg-red-50 border-red-200 text-red-700',
        };
        const icons = { Info: 'i', Warning: '!', Critical: '!!' };
        return (
          <div key={i} className={`flex items-start gap-2 p-2 rounded border mb-1 ${colors[alert.level]}`}>
            <span className="text-[13px] font-bold mt-0.5">{icons[alert.level]}</span>
            <div className="min-w-0">
              <p className="text-[13px] font-medium">{alert.category}</p>
              <p className="text-xs opacity-80">{alert.message}</p>
            </div>
          </div>
        );
      })}
    </div>
  );
}
