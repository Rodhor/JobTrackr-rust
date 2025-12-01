<script lang="ts">
    import { Button } from "$lib/components/ui/button";
    import { Badge } from "$lib/components/ui/badge";
    import { notes, deleteNote } from "$lib/stores/notes";
    import { NoteType, NoteTypeDisplay } from "$lib/types/enums";
    import PencilIcon from "lucide-svelte/icons/pencil";
    import FileTextIcon from "lucide-svelte/icons/file-text";
    import CalendarIcon from "lucide-svelte/icons/calendar";
    import DeleteAlert from "$lib/components/forms/utils/DeleteAlert.svelte";
    import { applications } from "$lib/stores/applications";
    import { jobListings } from "$lib/stores/jobListings";
    import { interactions } from "$lib/stores/interactions";
    import { people } from "$lib/stores/people";
    import { companies } from "$lib/stores/companies";

    let confirmDelete = $state(false);
    let selectedNoteId: number | null = $state(null);
    let selectedNoteTitle: string = $state("");

    $effect(() => {
        if (confirmDelete && selectedNoteId) {
            handleDelete(selectedNoteId);
            confirmDelete = false;
            selectedNoteId = null;
            selectedNoteTitle = "";
        }
    });

    async function handleDelete(id: number) {
        await deleteNote(id);
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

    function getLinkedEntities(note: (typeof $notes)[0]): string {
        const entities: string[] = [];

        if (note.companyId) {
            const company = $companies.find((c) => c.id === note.companyId);
            entities.push(`Company: ${company?.name ?? `#${note.companyId}`}`);
        }

        if (note.personId) {
            const person = $people.find((p) => p.id === note.personId);
            entities.push(
                `Person: ${person ? `${person.firstName} ${person.lastName}` : `#${note.personId}`}`,
            );
        }

        if (note.jobListingId) {
            const job = $jobListings.find((j) => j.id === note.jobListingId);
            entities.push(
                `Job Listing: ${job?.title ?? `#${note.jobListingId}`}`,
            );
        }

        if (note.applicationId) {
            const app = $applications.find((a) => a.id === note.applicationId);
            entities.push(
                `Application: ${app?.displayLabel ?? `#${note.applicationId}`}`,
            );
        }

        if (note.interactionId) {
            const interaction = $interactions.find(
                (i) => i.id === note.interactionId,
            );
            entities.push(
                `Interaction: ${interaction?.subject ?? `#${note.interactionId}`}`,
            );
        }

        return entities.length > 0 ? entities.join("\n") : "—";
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
                    <tr>
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
                            <div class="font-semibold">
                                {note.title || "—"}
                            </div>
                        </td>

                        <!-- Content -->
                        <td class="px-6 py-4 truncate max-w-[300px]">
                            {note.content || "—"}
                        </td>

                        <!-- Linked Entity -->
                        <td class="px-6 py-4 text-sm whitespace-pre-line">
                            {getLinkedEntities(note)}
                        </td>

                        <!-- Created Date -->
                        <td class="px-6 py-4">
                            <div class="flex items-center gap-2">
                                <CalendarIcon class="size-4" />
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
                                <DeleteAlert
                                    objectText="'{note.title || 'Note'}'"
                                    description="This will delete the note record."
                                    onDelete={async () => {
                                        await deleteNote(note.id!);
                                    }}
                                />
                            </div>
                        </td>
                    </tr>
                {/each}
            </tbody>
        </table>
    </div>
{/if}
