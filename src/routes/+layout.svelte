<script lang="ts">
    import "../app.css";
    import { ModeWatcher } from "mode-watcher";

    import AppSidebar from "$lib/components/app-sidebar.svelte";
    import * as Sidebar from "$lib/components/ui/sidebar/index.js";
    import TopBar from "$lib/components/topbar.svelte";
</script>

<ModeWatcher />

<Sidebar.Provider>
    <AppSidebar />
    <Sidebar.Inset>
        <TopBar />
        <div class="p-6">
            <slot />
        </div>
    </Sidebar.Inset>
</Sidebar.Provider>
