<script lang="ts">
	import "../app.css";
	import "@fontsource-variable/geist";

	import { ModeWatcher } from "mode-watcher";

	import Header from "$components/Header.svelte";
	import Updater from "$components/Updater.svelte";
	import { registerShortcuts } from "$scripts/shortcuts.svelte.js";
	import * as Tabs from "$lib/components/ui/tabs/index.js";

	import { onMount } from "svelte";
	import { markUiReady } from "$lib/deeplink";

	// Svelte 5: pega a função children() via $props()
	let { children } = $props();
	let tab = $state<"join" | "launch">("join");

	registerShortcuts();

	onMount(() => {
		markUiReady(); // ativa o listener e processa deep links pendentes
	});
</script>

<ModeWatcher />

<Tabs.Root bind:value={tab}>
	<Header />
	{@render children?.()}
</Tabs.Root>

<Updater />
