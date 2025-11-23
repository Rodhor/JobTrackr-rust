<script lang="ts">
    import * as Sidebar from "$lib/components/ui/sidebar/index.js";
    import * as Tooltip from "$lib/components/ui/tooltip/index.js";
    import HomeIcon from "@lucide/svelte/icons/home";
    import { useSidebar } from "$lib/components/ui/sidebar/index.js";

    const sidebar = useSidebar();
</script>

<Sidebar.Menu>
    <Sidebar.MenuItem>
        <Tooltip.Provider>
            <Tooltip.Root>
                <Tooltip.Trigger>
                    <Sidebar.MenuButton>
                        <a href={"/"} class="flex items-center gap-4">
                            <HomeIcon class="h-[1.2rem] w-[1.2rem]" />
                            {#if sidebar.state === "expanded"}
                                <span class="truncate">Home</span>
                            {/if}
                        </a>
                    </Sidebar.MenuButton>
                </Tooltip.Trigger>
                <Tooltip.Content>Go to overview</Tooltip.Content>
            </Tooltip.Root>
        </Tooltip.Provider>
    </Sidebar.MenuItem>
</Sidebar.Menu>
