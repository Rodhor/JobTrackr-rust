<script lang="ts" module>
    import AudioWaveformIcon from "@lucide/svelte/icons/audio-waveform";
    import BookOpenIcon from "@lucide/svelte/icons/book-open";
    import BotIcon from "@lucide/svelte/icons/bot";
    import CommandIcon from "@lucide/svelte/icons/command";
    import GalleryVerticalEndIcon from "@lucide/svelte/icons/gallery-vertical-end";
    import Settings2Icon from "@lucide/svelte/icons/settings-2";
    import SquareTerminalIcon from "@lucide/svelte/icons/square-terminal";
</script>

<script lang="ts">
    import * as Sidebar from "$lib/components/ui/sidebar/index.js";
    import type { ComponentProps } from "svelte";
    import ThemeToggle from "./theme-toggle.svelte";
    import NavWorkspace from "./nav-workspace.svelte";
    import NavHome from "./nav-home.svelte";
    import NavExports from "./nav-exports.svelte";
    import Separator from "./ui/separator/separator.svelte";

    let {
        ref = $bindable(null),
        collapsible = "icon",
        ...restProps
    }: ComponentProps<typeof Sidebar.Root> = $props();
</script>

<Sidebar.Root {collapsible} {...restProps}>
    <Sidebar.Header>
        <NavHome />
    </Sidebar.Header>
    <Sidebar.Content>
        <NavWorkspace />
        <Separator />
        <NavExports />
    </Sidebar.Content>
    <Sidebar.Footer>
        <ThemeToggle />
    </Sidebar.Footer>
    <Sidebar.Rail />
</Sidebar.Root>
