<script lang="ts">
    import * as Sidebar from "$lib/components/ui/sidebar/index.js";
    import * as Tooltip from "$lib/components/ui/tooltip/index.js";
    import { useSidebar } from "$lib/components/ui/sidebar";

    import LayoutDashboardIcon from "@lucide/svelte/icons/layout-dashboard";
    import BriefcaseIcon from "@lucide/svelte/icons/briefcase";
    import Building2Icon from "@lucide/svelte/icons/building-2";
    import UsersIcon from "@lucide/svelte/icons/users";
    import PhoneCallIcon from "@lucide/svelte/icons/phone-call";
    import StickyNoteIcon from "@lucide/svelte/icons/sticky-note";
    import ListIcon from "@lucide/svelte/icons/list";

    import { base } from "$app/paths";

    const sidebar = useSidebar();
</script>

<Sidebar.Group>
    <Sidebar.GroupLabel>Workspace</Sidebar.GroupLabel>

    <Sidebar.Menu>
        <!-- Dashboard -->
        <Sidebar.MenuItem>
            <Tooltip.Provider>
                <Tooltip.Root>
                    <Tooltip.Trigger>
                        <Sidebar.MenuButton>
                            <a
                                href={`${base}/dashboard`}
                                class="flex items-center gap-4"
                            >
                                <LayoutDashboardIcon class="h-4 w-4" />
                                {#if sidebar.state === "expanded"}
                                    <span class="truncate">Dashboard</span>
                                {/if}
                            </a>
                        </Sidebar.MenuButton>
                    </Tooltip.Trigger>
                    <Tooltip.Content
                        >View overall activity and stats</Tooltip.Content
                    >
                </Tooltip.Root>
            </Tooltip.Provider>
        </Sidebar.MenuItem>

        <!-- Job Listings -->
        <Sidebar.MenuItem>
            <Tooltip.Provider>
                <Tooltip.Root>
                    <Tooltip.Trigger>
                        <Sidebar.MenuButton>
                            <a
                                href={`${base}/jobListings`}
                                class="flex items-center gap-4"
                            >
                                <ListIcon class="h-4 w-4" />
                                {#if sidebar.state === "expanded"}
                                    <span class="truncate">Job Listings</span>
                                {/if}
                            </a>
                        </Sidebar.MenuButton>
                    </Tooltip.Trigger>
                    <Tooltip.Content
                        >Browse and manage job listings</Tooltip.Content
                    >
                </Tooltip.Root>
            </Tooltip.Provider>
        </Sidebar.MenuItem>

        <!-- Applications -->
        <Sidebar.MenuItem>
            <Tooltip.Provider>
                <Tooltip.Root>
                    <Tooltip.Trigger>
                        <Sidebar.MenuButton>
                            <a
                                href={`${base}/applications`}
                                class="flex items-center gap-4"
                            >
                                <BriefcaseIcon class="h-4 w-4" />
                                {#if sidebar.state === "expanded"}
                                    <span class="truncate">Applications</span>
                                {/if}
                            </a>
                        </Sidebar.MenuButton>
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
                        <Sidebar.MenuButton>
                            <a
                                href={`${base}/companies`}
                                class="flex items-center gap-4"
                            >
                                <Building2Icon class="h-4 w-4" />
                                {#if sidebar.state === "expanded"}
                                    <span class="truncate">Companies</span>
                                {/if}
                            </a>
                        </Sidebar.MenuButton>
                    </Tooltip.Trigger>
                    <Tooltip.Content
                        >View and organize company profiles</Tooltip.Content
                    >
                </Tooltip.Root>
            </Tooltip.Provider>
        </Sidebar.MenuItem>

        <!-- People -->
        <Sidebar.MenuItem>
            <Tooltip.Provider>
                <Tooltip.Root>
                    <Tooltip.Trigger>
                        <Sidebar.MenuButton>
                            <a
                                href={`${base}/people`}
                                class="flex items-center gap-4"
                            >
                                <UsersIcon class="h-4 w-4" />
                                {#if sidebar.state === "expanded"}
                                    <span class="truncate">People</span>
                                {/if}
                            </a>
                        </Sidebar.MenuButton>
                    </Tooltip.Trigger>
                    <Tooltip.Content
                        >View and manage people profiles</Tooltip.Content
                    >
                </Tooltip.Root>
            </Tooltip.Provider>
        </Sidebar.MenuItem>

        <!-- Interactions -->
        <Sidebar.MenuItem>
            <Tooltip.Provider>
                <Tooltip.Root>
                    <Tooltip.Trigger>
                        <Sidebar.MenuButton>
                            <a
                                href={`${base}/contacts`}
                                class="flex items-center gap-4"
                            >
                                <PhoneCallIcon class="h-4 w-4" />
                                {#if sidebar.state === "expanded"}
                                    <span class="truncate">Interactions</span>
                                {/if}
                            </a>
                        </Sidebar.MenuButton>
                    </Tooltip.Trigger>
                    <Tooltip.Content
                        >Track interviews, calls, and meetings</Tooltip.Content
                    >
                </Tooltip.Root>
            </Tooltip.Provider>
        </Sidebar.MenuItem>

        <!-- Notes -->
        <Sidebar.MenuItem>
            <Tooltip.Provider>
                <Tooltip.Root>
                    <Tooltip.Trigger>
                        <Sidebar.MenuButton>
                            <a
                                href={`${base}/contactNotes`}
                                class="flex items-center gap-4"
                            >
                                <StickyNoteIcon class="h-4 w-4" />
                                {#if sidebar.state === "expanded"}
                                    <span class="truncate">Notes</span>
                                {/if}
                            </a>
                        </Sidebar.MenuButton>
                    </Tooltip.Trigger>
                    <Tooltip.Content>Manage notes and reminders</Tooltip.Content
                    >
                </Tooltip.Root>
            </Tooltip.Provider>
        </Sidebar.MenuItem>
    </Sidebar.Menu>
</Sidebar.Group>
