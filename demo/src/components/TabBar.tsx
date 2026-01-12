import { Component } from "solid-js";

interface TabBarProps {
  tabs: { id: string; label: string; count?: number }[];
  activeTab: string;
  onTabChange: (id: string) => void;
}

export const TabBar: Component<TabBarProps> = (props) => {
  return (
    <div class="flex bg-neutral-50 dark:bg-neutral-900 overflow-x-auto">
      {props.tabs.map((tab) => (
        <button
          class={`px-4 py-3 text-sm font-medium transition-colors whitespace-nowrap flex items-center gap-2 ${
            props.activeTab === tab.id
              ? "bg-white dark:bg-neutral-950 text-accent"
              : "bg-transparent text-neutral-500 dark:text-neutral-500 hover:text-neutral-700 dark:hover:text-neutral-300"
          }`}
          onClick={() => props.onTabChange(tab.id)}
        >
          {tab.label}
          {tab.count !== undefined && tab.count > 0 && (
            <span class="bg-amber-100 dark:bg-amber-900/50 text-amber-600 dark:text-amber-400 text-[10px] px-1.5 py-0.5 rounded-full font-bold">
              {tab.count}
            </span>
          )}
        </button>
      ))}
    </div>
  );
};
