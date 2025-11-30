<script lang="ts">
    import { interactions, deleteInteraction } from "$lib/stores/interactions";
    import { Button } from "$lib/components/ui/button";
    import { Badge } from "$lib/components/ui/badge";
    import type { Interaction } from "$lib/types/interaction";
    import { InteractionType, InteractionTypeDisplay } from "$lib/types/enums";
    import Trash2Icon from "lucide-svelte/icons/trash-2";
    import PencilIcon from "lucide-svelte/icons/pencil";
    import MessageSquareIcon from "lucide-svelte/icons/message-square";
    import CalendarIcon from "lucide-svelte/icons/calendar";

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

    const typeColorMap: Record<InteractionType, string> = {
        [InteractionType.Email]: "bg-blue-100 text-blue-800",
        [InteractionType.Phone]: "bg-yellow-100 text-yellow-800",
        [InteractionType.Interview]: "bg-purple-100 text-purple-800",
        [InteractionType.Meeting]: "bg-green-100 text-green-800",
        [InteractionType.FollowUp]: "bg-teal-100 text-teal-800",
        [InteractionType.OfferDiscussion]: "bg-orange-100 text-orange-800",
        [InteractionType.Other]: "bg-gray-100 text-gray-800",
    };

    function typeColor(type?: InteractionType): string {
        return type ? typeColorMap[type] : "bg-gray-200 text-gray-800";
    }
</script>

<!-- ----------------------------------------------------------
Header
----------------------------------------------------------- -->
<div class="mb-8 flex items-center justify-between">
    <div>
        <h1 class="text-3xl font-bold tracking-tight">Interactions</h1>
        <p class="text-muted-foreground mt-1">
            Track and manage all your communications
        </p>
    </div>
    <Button href="/interactions/create" size="lg">+ New Interaction</Button>
</div>

<!-- ----------------------------------------------------------
Stats Card
----------------------------------------------------------- -->
<div class="mb-8 rounded-lg border border-border bg-background p-4">
    <div class="flex items-center justify-between">
        <div>
            <div class="text-sm font-medium text-muted-foreground mb-1">
                Total Interactions
            </div>
            <div class="text-3xl font-bold">{$interactions.length}</div>
        </div>
        <MessageSquareIcon class="size-12 text-muted-foreground opacity-30" />
    </div>
</div>

<!-- ----------------------------------------------------------
Empty State
----------------------------------------------------------- -->
{#if $interactions.length === 0}
    <div
        class="rounded-lg border border-dashed border-border bg-muted/30 p-12 text-center"
    >
        <MessageSquareIcon
            class="mx-auto mb-4 size-12 text-muted-foreground opacity-50"
        />
        <h3 class="font-semibold text-foreground mb-1">No interactions yet</h3>
        <p class="text-sm text-muted-foreground mb-4">
            Start logging your communications and interactions
        </p>
        <Button href="/interactions/create">Create First Interaction</Button>
    </div>
{:else}
    <!-- ----------------------------------------------------------
    Interactions Table
    ----------------------------------------------------------- -->
    <div
        class="overflow-hidden rounded-lg border border-border bg-background shadow-sm"
    >
        <table class="w-full text-sm">
            <thead
                class="bg-muted/50 text-left text-xs uppercase tracking-wider text-muted-foreground border-b border-border"
            >
                <tr>
                    <th class="px-6 py-4 font-semibold">Type</th>
                    <th class="px-6 py-4 font-semibold">Date</th>
                    <th class="px-6 py-4 font-semibold">Subject</th>
                    <th class="px-6 py-4 font-semibold">Medium</th>
                    <th class="px-6 py-4 font-semibold">Application</th>
                    <th class="px-6 py-4 text-right font-semibold">Actions</th>
                </tr>
            </thead>

            <tbody>
                {#each $interactions as interaction (interaction.id)}
                    <tr
                        class="border-t border-border hover:bg-muted/50 transition-colors"
                    >
                        <!-- Type -->
                        <td class="px-6 py-4">
                            <Badge
                                class={typeColor(interaction.interactionType)}
                            >
                                {InteractionTypeDisplay[
                                    interaction.interactionType
                                ]}
                            </Badge>
                        </td>

                        <!-- Date -->
                        <td class="px-6 py-4">
                            <div
                                class="flex items-center gap-2 text-foreground"
                            >
                                <CalendarIcon
                                    class="size-4 text-muted-foreground"
                                />
                                {formatDate(interaction.interactionDate)}
                            </div>
                        </td>

                        <!-- Subject -->
                        <td class="px-6 py-4">
                            <div
                                class="font-semibold text-foreground truncate max-w-[180px]"
                            >
                                {interaction.subject || "—"}
                            </div>
                        </td>

                        <!-- Medium -->
                        <td class="px-6 py-4 text-muted-foreground">
                            {interaction.medium || "—"}
                        </td>

                        <!-- Application -->
                        <td class="px-6 py-4 text-muted-foreground">
                            {#if interaction.applicationId}
                                Application #{interaction.applicationId}
                            {:else}
                                —
                            {/if}
                        </td>

                        <!-- Actions -->
                        <td class="px-6 py-4">
                            <div class="flex justify-end gap-2">
                                <Button
                                    size="sm"
                                    variant="outline"
                                    class="gap-2"
                                    href="/interactions/{interaction.id}"
                                >
                                    <PencilIcon class="size-4" />
                                    Edit
                                </Button>
                                <Button
                                    size="sm"
                                    variant="destructive"
                                    class="gap-2"
                                    onclick={() =>
                                        interaction.id &&
                                        handleDelete(interaction.id)}
                                >
                                    <Trash2Icon class="size-4" />
                                    Delete
                                </Button>
                            </div>
                        </td>
                    </tr>
                {/each}
            </tbody>
        </table>
    </div>
{/if}
