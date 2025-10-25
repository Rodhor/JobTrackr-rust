<script lang="ts">
    import * as Sidebar from "$lib/components/ui/sidebar/index.js";
    import * as Tooltip from "$lib/components/ui/tooltip/index.js";
    import { useSidebar } from "$lib/components/ui/sidebar";

    import FileDownIcon from "@lucide/svelte/icons/file-down";
    import FileJsonIcon from "@lucide/svelte/icons/file-json";
    import FileSpreadsheetIcon from "@lucide/svelte/icons/file-spreadsheet";
    import DatabaseIcon from "@lucide/svelte/icons/database";

    import { base } from "$app/paths";

    const sidebar = useSidebar();
</script>

<Sidebar.Group>
    <Sidebar.GroupLabel>Exports</Sidebar.GroupLabel>

    <Sidebar.Menu>
        <!-- CSV -->
        <Sidebar.MenuItem>
            <Tooltip.Provider>
                <Tooltip.Root>
                    <Tooltip.Trigger>
                        <Sidebar.MenuButton>
                            <a
                                href={`${base}/export/csv`}
                                class="flex items-center gap-4"
                            >
                                <FileSpreadsheetIcon class="h-4 w-4" />
                                {#if sidebar.state === "expanded"}
                                    <span class="truncate">Export CSV</span>
                                {/if}
                            </a>
                        </Sidebar.MenuButton>
                    </Tooltip.Trigger>
                    <Tooltip.Content
                        >Export all data as CSV file</Tooltip.Content
                    >
                </Tooltip.Root>
            </Tooltip.Provider>
        </Sidebar.MenuItem>

        <!-- Excel -->
        <Sidebar.MenuItem>
            <Tooltip.Provider>
                <Tooltip.Root>
                    <Tooltip.Trigger>
                        <Sidebar.MenuButton>
                            <a
                                href={`${base}/export/excel`}
                                class="flex items-center gap-4"
                            >
                                <FileDownIcon class="h-4 w-4" />
                                {#if sidebar.state === "expanded"}
                                    <span class="truncate">Export Excel</span>
                                {/if}
                            </a>
                        </Sidebar.MenuButton>
                    </Tooltip.Trigger>
                    <Tooltip.Content
                        >Export all data as Excel workbook</Tooltip.Content
                    >
                </Tooltip.Root>
            </Tooltip.Provider>
        </Sidebar.MenuItem>

        <!-- JSON -->
        <Sidebar.MenuItem>
            <Tooltip.Provider>
                <Tooltip.Root>
                    <Tooltip.Trigger>
                        <Sidebar.MenuButton>
                            <a
                                href={`${base}/export/json`}
                                class="flex items-center gap-4"
                            >
                                <FileJsonIcon class="h-4 w-4" />
                                {#if sidebar.state === "expanded"}
                                    <span class="truncate">Export JSON</span>
                                {/if}
                            </a>
                        </Sidebar.MenuButton>
                    </Tooltip.Trigger>
                    <Tooltip.Content
                        >Export all data in JSON format</Tooltip.Content
                    >
                </Tooltip.Root>
            </Tooltip.Provider>
        </Sidebar.MenuItem>

        <!-- Database Backup -->
        <Sidebar.MenuItem>
            <Tooltip.Provider>
                <Tooltip.Root>
                    <Tooltip.Trigger>
                        <Sidebar.MenuButton>
                            <a
                                href={`${base}/export/backup`}
                                class="flex items-center gap-4"
                            >
                                <DatabaseIcon class="h-4 w-4" />
                                {#if sidebar.state === "expanded"}
                                    <span class="truncate"
                                        >Export DB Backup</span
                                    >
                                {/if}
                            </a>
                        </Sidebar.MenuButton>
                    </Tooltip.Trigger>
                    <Tooltip.Content
                        >Create a full database backup file</Tooltip.Content
                    >
                </Tooltip.Root>
            </Tooltip.Provider>
        </Sidebar.MenuItem>
    </Sidebar.Menu>
</Sidebar.Group>
