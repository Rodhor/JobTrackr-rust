<script lang="ts">
    import * as Sidebar from "$lib/components/ui/sidebar/index.js";

    import LayoutDashboardIcon from "@lucide/svelte/icons/layout-dashboard";
    import BriefcaseIcon from "@lucide/svelte/icons/briefcase";
    import Building2Icon from "@lucide/svelte/icons/building-2";
    import StickyNoteIcon from "@lucide/svelte/icons/sticky-note";
    import * as Tooltip from "$lib/components/ui/tooltip/index.js";

    import { base } from "$app/paths"; // ensures correct relative pathing for Tauri
</script>

<Sidebar.Group>
    <Sidebar.GroupLabel>Workspace</Sidebar.GroupLabel>

    <Sidebar.Menu>
        <!-- Dashboard -->
        <Sidebar.MenuItem>
            <Tooltip.Provider>
                <Tooltip.Root>
                    <Tooltip.Trigger>
                        <a
                            href={`${base}/dashboard`}
                            class="flex items-center gap-2 w-full text-left px-2 py-2 rounded-md hover:bg-accent transition-colors"
                        >
                            <LayoutDashboardIcon class="h-4 w-4 mr-2" />
                            Dashboard
                        </a>
                    </Tooltip.Trigger>
                    <Tooltip.Content
                        >View overall activity and stats</Tooltip.Content
                    >
                </Tooltip.Root>
            </Tooltip.Provider>
        </Sidebar.MenuItem>

        <!-- Applications -->
        <Sidebar.MenuItem>
            <Tooltip.Provider>
                <Tooltip.Root>
                    <Tooltip.Trigger>
                        <a
                            href={`${base}/applications`}
                            class="flex items-center gap-2 w-full text-left px-2 py-2 rounded-md hover:bg-accent transition-colors"
                        >
                            <BriefcaseIcon class="h-4 w-4 mr-2" />
                            Applications
                        </a>
                    </Tooltip.Trigger>
                    <Tooltip.Content
                        >Manage your job applications</Tooltip.Content
                    >
                </Tooltip.Root>
            </Tooltip.Provider>
        </Sidebar.MenuItem>

        <!-- Companies -->
        <Sidebar.MenuItem>
            <Tooltip.Provider>
                <Tooltip.Root>
                    <Tooltip.Trigger>
                        <a
                            href={`${base}/companies`}
                            class="flex items-center gap-2 w-full text-left px-2 py-2 rounded-md hover:bg-accent transition-colors"
                        >
                            <Building2Icon class="h-4 w-4 mr-2" />
                            Companies
                        </a>
                    </Tooltip.Trigger>
                    <Tooltip.Content
                        >View and organize company profiles</Tooltip.Content
                    >
                </Tooltip.Root>
            </Tooltip.Provider>
        </Sidebar.MenuItem>

        <!-- Notes -->
        <Sidebar.MenuItem>
            <Tooltip.Provider>
                <Tooltip.Root>
                    <Tooltip.Trigger>
                        <a
                            href={`${base}/notes`}
                            class="flex items-center gap-2 w-full text-left px-2 py-2 rounded-md hover:bg-accent transition-colors"
                        >
                            <StickyNoteIcon class="h-4 w-4 mr-2" />
                            Notes
                        </a>
                    </Tooltip.Trigger>
                    <Tooltip.Content>Manage notes and reminders</Tooltip.Content
                    >
                </Tooltip.Root>
            </Tooltip.Provider>
        </Sidebar.MenuItem>
    </Sidebar.Menu>
</Sidebar.Group>
