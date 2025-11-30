<script lang="ts">
    import { Button } from "$lib/components/ui/button";
    import { Badge } from "$lib/components/ui/badge";
    import { notes, deleteNote } from "$lib/stores/notes";
    import { NoteType, NoteTypeDisplay } from "$lib/types/enums";
    import Trash2Icon from "lucide-svelte/icons/trash-2";
    import PencilIcon from "lucide-svelte/icons/pencil";
    import FileTextIcon from "lucide-svelte/icons/file-text";
    import CalendarIcon from "lucide-svelte/icons/calendar";

    async function handleDelete(id: number) {
        try {
            await deleteNote(id);
        } catch (err) {
            console.error("Failed to delete note:", err);
        }
    }

    function formatDate(dateStr?: string) {
        if (!dateStr) return "—";
        const date = new Date(dateStr);
        if (isNaN(date.getTime())) return dateStr;
        return date.toLocaleDateString(undefined, {
            year: "numeric",
            month: "short",
            day: "numeric",
        });
    }

    const typeColorMap: Record<NoteType, string> = {
        [NoteType.General]: "bg-yellow-100 text-yellow-800",
        [NoteType.Feedback]: "bg-green-100 text-green-800",
        [NoteType.Reminder]: "bg-blue-100 text-blue-800",
        [NoteType.Summary]: "bg-purple-100 text-purple-800",
        [NoteType.Other]: "bg-gray-100 text-gray-800",
    };

    function typeColor(type?: NoteType): string {
        return type ? typeColorMap[type] : "bg-gray-200 text-gray-800";
    }

    function getLinkedEntity(note: (typeof $notes)[0]): {
        label: string;
        type: string;
    } {
        if (note.applicationId)
            return {
                label: `Application #${note.applicationId}`,
                type: "application",
            };
        if (note.jobListingId)
            return { label: `Job Listing #${note.jobListingId}`, type: "job" };
        if (note.interactionId)
            return {
                label: `Interaction #${note.interactionId}`,
                type: "interaction",
            };
        if (note.personId)
            return { label: `Person #${note.personId}`, type: "person" };
        if (note.companyId)
            return { label: `Company #${note.companyId}`, type: "company" };
        return { label: "—", type: "none" };
    }
</script>

<!-- ----------------------------------------------------------
Header
----------------------------------------------------------- -->
<div class="mb-8 flex items-center justify-between">
    <div>
        <h1 class="text-3xl font-bold tracking-tight">Notes</h1>
        <p class="text-muted-foreground mt-1">
            Manage your notes and annotations
        </p>
    </div>
    <Button href="/notes/create" size="lg">+ New Note</Button>
</div>

<!-- ----------------------------------------------------------
Stats Card
----------------------------------------------------------- -->
<div class="mb-8 rounded-lg border border-border bg-background p-4">
    <div class="flex items-center justify-between">
        <div>
            <div class="text-sm font-medium text-muted-foreground mb-1">
                Total Notes
            </div>
            <div class="text-3xl font-bold">{$notes.length}</div>
        </div>
        <FileTextIcon class="size-12 text-muted-foreground opacity-30" />
    </div>
</div>

<!-- ----------------------------------------------------------
Empty State
----------------------------------------------------------- -->
{#if $notes.length === 0}
    <div
        class="rounded-lg border border-dashed border-border bg-muted/30 p-12 text-center"
    >
        <FileTextIcon
            class="mx-auto mb-4 size-12 text-muted-foreground opacity-50"
        />
        <h3 class="font-semibold text-foreground mb-1">No notes yet</h3>
        <p class="text-sm text-muted-foreground mb-4">
            Start creating notes to organize your thoughts
        </p>
        <Button href="/notes/create">Create First Note</Button>
    </div>
{:else}
    <!-- ----------------------------------------------------------
    Notes Table
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
                    <th class="px-6 py-4 font-semibold">Title</th>
                    <th class="px-6 py-4 font-semibold">Content</th>
                    <th class="px-6 py-4 font-semibold">Linked Entity</th>
                    <th class="px-6 py-4 font-semibold">Created</th>
                    <th class="px-6 py-4 text-right font-semibold">Actions</th>
                </tr>
            </thead>
            <tbody>
                {#each $notes as note (note.id)}
                    {@const linkedEntity = getLinkedEntity(note)}
                    <tr
                        class="border-t border-border hover:bg-muted/50 transition-colors"
                    >
                        <!-- Type -->
                        <td class="px-6 py-4">
                            <Badge class={typeColor(note.noteType)}>
                                {note.noteType
                                    ? NoteTypeDisplay[note.noteType]
                                    : "—"}
                            </Badge>
                        </td>

                        <!-- Title -->
                        <td class="px-6 py-4">
                            <div class="font-semibold text-foreground">
                                {note.title || "—"}
                            </div>
                        </td>

                        <!-- Content -->
                        <td
                            class="px-6 py-4 truncate max-w-[300px] text-muted-foreground"
                        >
                            {note.content || "—"}
                        </td>

                        <!-- Linked Entity -->
                        <td class="px-6 py-4 text-sm text-muted-foreground">
                            {linkedEntity.label}
                        </td>

                        <!-- Created Date -->
                        <td class="px-6 py-4">
                            <div
                                class="flex items-center gap-2 text-foreground"
                            >
                                <CalendarIcon
                                    class="size-4 text-muted-foreground"
                                />
                                {formatDate(note.createdAt)}
                            </div>
                        </td>

                        <!-- Actions -->
                        <td class="px-6 py-4">
                            <div class="flex justify-end gap-2">
                                <Button
                                    size="sm"
                                    variant="outline"
                                    class="gap-2"
                                    href="/notes/{note.id}"
                                >
                                    <PencilIcon class="size-4" />
                                    Edit
                                </Button>
                                <Button
                                    size="sm"
                                    variant="destructive"
                                    class="gap-2"
                                    onclick={() =>
                                        note.id && handleDelete(note.id)}
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
