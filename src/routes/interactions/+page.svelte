<script lang="ts">
    import { interactions, deleteInteraction } from "$lib/stores/interactions";
    import { Button } from "$lib/components/ui/button";
    import { Badge } from "$lib/components/ui/badge";
    import InteractionDialog from "$lib/components/formDialogs/InteractionDialog.svelte";
    import type { Interaction } from "$lib/types/interaction";
    import { InteractionType, InteractionTypeDisplay } from "$lib/types/enums";

    async function handleDelete(id: number) {
        await deleteInteraction(id);
    }

    function formatDate(dateStr: string) {
        if (!dateStr) return "—";
        const date = new Date(dateStr);
        if (isNaN(date.getTime())) return dateStr;
        return date.toLocaleDateString(undefined, {
            year: "numeric",
            month: "short",
            day: "numeric",
        });
    }

    function typeColor(type: InteractionType) {
        switch (type) {
            case InteractionType.Email:
                return "bg-blue-100 text-blue-800";
            case InteractionType.Phone:
                return "bg-yellow-100 text-yellow-800";
            case InteractionType.Interview:
                return "bg-purple-100 text-purple-800";
            case InteractionType.Meeting:
                return "bg-green-100 text-green-800";
            case InteractionType.FollowUp:
                return "bg-teal-100 text-teal-800";
            case InteractionType.OfferDiscussion:
                return "bg-orange-100 text-orange-800";
            default:
                return "bg-gray-200 text-gray-800";
        }
    }
</script>

<!-- ----------------------------------------------------------
Header
----------------------------------------------------------- -->
<div class="mb-6 flex items-center justify-between">
    <h1 class="text-2xl font-semibold tracking-tight">Interactions</h1>
    <Button href="/interactions/create">New Interaction</Button>
</div>

<!-- ----------------------------------------------------------
Interactions Table
----------------------------------------------------------- -->
<div
    class="overflow-hidden rounded-lg border border-border bg-background shadow-sm"
>
    <table class="min-w-full text-sm">
        <thead
            class="bg-muted/50 text-left text-xs uppercase tracking-wider text-muted-foreground"
        >
            <tr>
                <th class="px-4 py-3">Type</th>
                <th class="px-4 py-3">Date</th>
                <th class="px-4 py-3">Subject</th>
                <th class="px-4 py-3">Medium</th>
                <th class="px-4 py-3">Application</th>
                <th class="px-4 py-3 text-right">Actions</th>
            </tr>
        </thead>

        <tbody>
            {#each $interactions as i (i.id)}
                <tr class="border-t hover:bg-muted/30 transition-colors">
                    <td class="px-4 py-3">
                        <Badge class={typeColor(i.interactionType)}>
                            {InteractionTypeDisplay[i.interactionType]}
                        </Badge>
                    </td>
                    <td class="px-4 py-3">{formatDate(i.interactionDate)}</td>
                    <td
                        class="px-4 py-3 truncate max-w-[180px] text-muted-foreground"
                    >
                        {i.subject || "—"}
                    </td>
                    <td class="px-4 py-3">{i.medium || "—"}</td>
                    <td class="px-4 py-3">{i.applicationId || "—"}</td>
                    <td class="px-4 py-3 text-right flex justify-end gap-2">
                        <Button
                            size="sm"
                            variant="outline"
                            href="/interactions/{i.id}">Edit</Button
                        >
                        <Button
                            size="sm"
                            variant="destructive"
                            onclick={() => i.id && handleDelete(i.id)}
                        >
                            Delete
                        </Button>
                    </td>
                </tr>
            {/each}

            {#if $interactions.length === 0}
                <tr>
                    <td
                        colspan="6"
                        class="px-4 py-10 text-center text-sm text-muted-foreground"
                    >
                        No interactions yet.
                    </td>
                </tr>
            {/if}
        </tbody>
    </table>
</div>
