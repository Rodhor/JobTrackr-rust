<script lang="ts">
    import { onMount } from "svelte";
    import { Button } from "$lib/components/ui/button";
    import { Badge } from "$lib/components/ui/badge";
    import NoteDialog from "$lib/components/formDialogs/NoteDialog.svelte";
    import { notes, loadNotes, deleteNote } from "$lib/stores/notes";
    import { writable } from "svelte/store";
    import { NoteType, NoteTypeDisplay } from "$lib/types/enums";
    import type { Note } from "$lib/types/note";

    const dialogOpen = writable(false);
    const mode = writable<"create" | "edit">("create");
    const selectedNote = writable<Note | null>(null);

    onMount(loadNotes);

    function handleCreate() {
        selectedNote.set(null);
        mode.set("create");
        dialogOpen.set(true);
    }

    function handleEdit(note: Note) {
        selectedNote.set(note);
        mode.set("edit");
        dialogOpen.set(true);
    }

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

    function typeColor(type?: NoteType) {
        switch (type) {
            case NoteType.General:
                return "bg-yellow-100 text-yellow-800";
            case NoteType.Feedback:
                return "bg-green-100 text-green-800";
            case NoteType.Reminder:
                return "bg-blue-100 text-blue-800";
            case NoteType.Summary:
                return "bg-purple-100 text-purple-800";
            default:
                return "bg-gray-200 text-gray-800";
        }
    }
</script>

<!-- ----------------------------------------------------------
Header
----------------------------------------------------------- -->
<div class="mb-6 flex items-center justify-between">
    <h1 class="text-2xl font-semibold tracking-tight">Notes</h1>
    <Button onclick={handleCreate}>New Note</Button>
</div>

<NoteDialog bind:open={$dialogOpen} mode={$mode} existingNote={$selectedNote} />

<!-- ----------------------------------------------------------
Notes Table
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
                <th class="px-4 py-3">Title</th>
                <th class="px-4 py-3">Content</th>
                <th class="px-4 py-3">Linked Entity</th>
                <th class="px-4 py-3">Created</th>
                <th class="px-4 py-3 text-right">Actions</th>
            </tr>
        </thead>
        <tbody>
            {#each $notes as n (n.id)}
                <tr class="border-t hover:bg-muted/30 transition-colors">
                    <td class="px-4 py-3">
                        <Badge class={typeColor(n.noteType)}>
                            {n.noteType ? NoteTypeDisplay[n.noteType] : "—"}
                        </Badge>
                    </td>
                    <td class="px-4 py-3 font-medium text-foreground"
                        >{n.title || "—"}</td
                    >
                    <td
                        class="px-4 py-3 truncate max-w-[300px] text-muted-foreground"
                        >{n.content || "—"}</td
                    >
                    <td class="px-4 py-3 text-muted-foreground">
                        {#if n.applicationId}
                            Application #{n.applicationId}
                        {:else if n.jobListingId}
                            Job Listing #{n.jobListingId}
                        {:else if n.interactionId}
                            Interaction #{n.interactionId}
                        {:else if n.personId}
                            Person #{n.personId}
                        {:else if n.companyId}
                            Company #{n.companyId}
                        {:else}
                            —
                        {/if}
                    </td>
                    <td class="px-4 py-3">{formatDate(n.createdAt)}</td>
                    <td class="px-4 py-3 text-right flex justify-end gap-2">
                        <Button
                            size="sm"
                            variant="outline"
                            onclick={() => handleEdit(n)}>Edit</Button
                        >
                        <Button
                            size="sm"
                            variant="destructive"
                            onclick={() => handleDelete(n.id)}>Delete</Button
                        >
                    </td>
                </tr>
            {/each}

            {#if $notes.length === 0}
                <tr>
                    <td
                        colspan="6"
                        class="px-4 py-10 text-center text-sm text-muted-foreground"
                    >
                        No notes yet.
                    </td>
                </tr>
            {/if}
        </tbody>
    </table>
</div>
