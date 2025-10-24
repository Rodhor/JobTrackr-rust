<script lang="ts">
    import * as Sidebar from "$lib/components/ui/sidebar/index.js";

    import FileDownIcon from "@lucide/svelte/icons/file-down";
    import FileJsonIcon from "@lucide/svelte/icons/file-json";
    import FileSpreadsheetIcon from "@lucide/svelte/icons/file-spreadsheet";
    import DatabaseIcon from "@lucide/svelte/icons/database";
    import * as Tooltip from "$lib/components/ui/tooltip/index.js";
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
                            <FileSpreadsheetIcon class="h-4 w-4 mr-2" />
                            Export CSV
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
                            <FileDownIcon class="h-4 w-4 mr-2" />
                            Export Excel
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
                            <FileJsonIcon class="h-4 w-4 mr-2" />
                            Export JSON
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
                            <DatabaseIcon class="h-4 w-4 mr-2" />
                            Export DB Backup
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
