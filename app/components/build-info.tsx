export function BuildInfo() {
  return (
    <div className="fixed bottom-2 right-2 text-xs text-gray-400 font-mono">
      v{__BUILD_VERSION__}
    </div>
  );
}
