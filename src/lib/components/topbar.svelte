<script>
    import { Separator } from "$lib/components/ui/separator/index.js";
    import * as Sidebar from "$lib/components/ui/sidebar/index.js";
</script>

<header
    class="group-has-data-[collapsible=icon]/sidebar-wrapper:h-12 flex h-16 shrink-0 items-center transition-[width,height] ease-linear border-b border-border bg-background"
>
    <div class="flex items-center gap-2 px-3">
        <Sidebar.Trigger
            class="flex items-center justify-center h-9 w-9 rounded-md hover:bg-accent transition-colors"
        />

        <Separator
            orientation="vertical"
            class="h-6 mx-2 data-[orientation=vertical]:h-6"
        />

        <span class="text-sm font-medium tracking-tight">JobTrackr</span>
    </div>
</header>
