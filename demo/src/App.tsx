import { Component, createSignal, createEffect, onMount, Show } from "solid-js";
import { formatWithWarnings, FormatOptions, Warning } from "@hongdown/wasm";
import sampleMarkdown from "./sample.md?raw";
import { OptionsPanel } from "./components/Options";
import { TabBar } from "./components/TabBar";

const App: Component = () => {
  const [input, setInput] = createSignal(sampleMarkdown);
  const [output, setOutput] = createSignal("");
  const [warnings, setWarnings] = createSignal<Warning[]>([]);
  const [options, setOptions] = createSignal<FormatOptions>({});
  const [activeTab, setActiveTab] = createSignal("editor");
  const [isMobile, setIsMobile] = createSignal(false);

  // Check for mobile on mount and resize
  onMount(() => {
    const checkMobile = () => setIsMobile(window.innerWidth < 1024);
    checkMobile();
    window.addEventListener("resize", checkMobile);
    return () => window.removeEventListener("resize", checkMobile);
  });

  // Re-format when input or options change
  createEffect(async () => {
    try {
      const result = await formatWithWarnings(input(), options());
      setOutput(result.output);
      setWarnings(result.warnings);
    } catch (e) {
      console.error("Formatting error:", e);
    }
  });

  const resetOptions = () => setOptions({});

  return (
    <div class="flex flex-col h-screen overflow-hidden">
      {/* Header */}
      <header class="bg-neutral-50 dark:bg-neutral-900 px-4 py-3 flex items-center justify-between z-10">
        <div class="flex items-center gap-3">
          <div class="i-carbon-document-markdown w-6 h-6 text-accent" />
          <h1 class="font-bold text-lg tracking-tight">Hongdown Demo</h1>
        </div>
        <div class="flex items-center gap-4">
          <a
            href="https://github.com/dahlia/hongdown"
            target="_blank"
            class="text-neutral-500 hover:text-neutral-900 dark:hover:text-neutral-100"
          >
            <div class="i-carbon-logo-github w-5 h-5" />
          </a>
        </div>
      </header>

      {/* Main Content */}
      <main class="flex-1 flex flex-col min-h-0 relative">
        <Show when={isMobile()}>
          <TabBar
            tabs={[
              { id: "editor", label: "Editor" },
              { id: "output", label: "Output" },
              { id: "warnings", label: "Warnings", count: warnings().length },
            ]}
            activeTab={activeTab()}
            onTabChange={setActiveTab}
          />
        </Show>

        <div class={`flex-1 flex min-h-0 ${isMobile() ? "flex-col overflow-hidden" : "flex-row overflow-hidden"}`}>
          {/* Editor */}
          <Show when={!isMobile() || activeTab() === "editor"}>
            <section
              class={`${isMobile() ? "flex-1 w-full min-h-0" : "flex-1 min-h-0"} flex flex-col bg-white dark:bg-neutral-950`}
            >
              <textarea
                class="flex-1 p-6 font-mono text-sm resize-none focus:outline-none bg-transparent text-neutral-900 dark:text-neutral-100 placeholder:text-neutral-400 dark:placeholder:text-neutral-600"
                value={input()}
                onInput={(e) => setInput(e.currentTarget.value)}
                placeholder="Enter Markdown here..."
                spellcheck={false}
              />
            </section>
          </Show>

          {/* Preview/Output */}
          <Show when={!isMobile() || activeTab() === "output"}>
            <section
              class={`${isMobile() ? "flex-1 w-full min-h-0" : "flex-1 min-h-0"} flex flex-col bg-neutral-100 dark:bg-neutral-900`}
            >
              <div class="flex-1 overflow-auto p-6 relative min-h-0">
                <div class="absolute top-4 right-4 z-10">
                  <button
                    class="btn btn-outline py-1.5 px-3 text-xs flex items-center gap-1.5 backdrop-blur-sm bg-white/80 dark:bg-neutral-900/80"
                    onClick={() => {
                      navigator.clipboard.writeText(output());
                    }}
                  >
                    <div class="i-carbon-copy w-3.5 h-3.5" />
                    Copy
                  </button>
                </div>
                <pre class="m-0 font-mono text-sm whitespace-pre-wrap select-text cursor-text text-neutral-900 dark:text-neutral-200">
                  {output()}
                </pre>
              </div>
            </section>
          </Show>

          {/* Warnings (Mobile only tab) */}
          <Show when={isMobile() && activeTab() === "warnings"}>
            <section class="flex-1 w-full flex flex-col bg-white dark:bg-neutral-950 p-4 min-h-0 overflow-auto">
              <h2 class="font-bold mb-4">Warnings ({warnings().length})</h2>
              <Show
                when={warnings().length > 0}
                fallback={
                  <div class="flex flex-col items-center justify-center flex-1 text-neutral-400">
                    <div class="i-carbon-checkmark-filled w-12 h-12 mb-2 text-green-500 opacity-50" />
                    <p>No warnings found!</p>
                  </div>
                }
              >
                <div class="flex flex-col gap-2">
                  {warnings().map((w) => (
                    <div class="p-3 bg-amber-50 dark:bg-amber-900/20 border border-amber-200 dark:border-amber-800 rounded-md flex gap-3">
                      <div class="i-carbon-warning-alt text-amber-500 w-5 h-5 flex-shrink-0" />
                      <div class="text-sm">
                        <div class="font-bold text-amber-700 dark:text-amber-400">
                          Line {w.line}
                        </div>
                        <div class="text-neutral-700 dark:text-neutral-300">
                          {w.message}
                        </div>
                      </div>
                    </div>
                  ))}
                </div>
              </Show>
            </section>
          </Show>
        </div>

        {/* Floating Warnings Badge for Desktop */}
        <Show when={!isMobile() && warnings().length > 0}>
          <div class="absolute right-8 bottom-8 z-20">
             <div class="bg-amber-100 dark:bg-amber-900 border border-amber-300 dark:border-amber-700 rounded-lg shadow-lg p-3 max-w-sm">
                <div class="flex items-center gap-2 mb-2 text-amber-700 dark:text-amber-300 font-bold text-sm">
                   <div class="i-carbon-warning-alt w-4 h-4" />
                   Warnings ({warnings().length})
                </div>
                <div class="max-h-40 overflow-y-auto flex flex-col gap-2">
                  {warnings().slice(0, 3).map(w => (
                    <div class="text-xs text-neutral-600 dark:text-neutral-400 border-l-2 border-amber-400 pl-2">
                      Line {w.line}: {w.message}
                    </div>
                  ))}
                  {warnings().length > 3 && (
                    <div class="text-xs text-neutral-500 italic">
                      and {warnings().length - 3} more...
                    </div>
                  )}
                </div>
             </div>
          </div>
        </Show>

        <OptionsPanel
          options={options()}
          setOptions={setOptions}
          resetOptions={resetOptions}
        />
      </main>
    </div>
  );
};

export default App;
