<script lang="ts">
    import * as Sidebar from "$lib/components/ui/sidebar/index.js";
    import HomeIcon from "@lucide/svelte/icons/home";
    import { goto } from "$app/navigation";
    import * as Tooltip from "$lib/components/ui/tooltip/index.js";
    import { base } from "$app/paths";
</script>

<Sidebar.Menu>
    <Sidebar.MenuItem>
        <Tooltip.Provider>
            <Tooltip.Root>
                <Tooltip.Trigger>
                    <Sidebar.MenuButton>
                        <a href={`${base}/`} class="flex items-center gap-2">
                            <HomeIcon class="h-[1.2rem] w-[1.2rem]" />
                            Home
                        </a>
                    </Sidebar.MenuButton>
                </Tooltip.Trigger>
                <Tooltip.Content>Go to overview</Tooltip.Content>
            </Tooltip.Root>
        </Tooltip.Provider>
    </Sidebar.MenuItem>
</Sidebar.Menu>
