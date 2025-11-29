<script lang="ts">
    import { goto } from "$app/navigation";
    import { Button } from "$lib/components/ui/button";
    import { companies } from "$lib/stores/companies";
    import { createNote, notes, updateNote } from "$lib/stores/notes";
    import { people } from "$lib/stores/people";
    import { jobListings } from "$lib/stores/jobListings";
    import { applications } from "$lib/stores/applications";
    import { interactions } from "$lib/stores/interactions";
    import { NoteType } from "$lib/types/enums";
    import type { Note } from "$lib/types/note";
    import CustomEnumSelector from "../formDialogs/utils/CustomEnumSelector.svelte";
    import CustomIDSelectCreate from "../formDialogs/utils/CustomIDSelectCreate.svelte";
    import { Input } from "../ui/input";
    import { Label } from "../ui/label";
    import { Textarea } from "../ui/textarea";
    import { newlyCreatedNoteId } from "$lib/stores/formState";

    let { noteID, caller }: { noteID?: number; caller?: string | null } =
        $props();

    const companyItems = $derived(
        $companies
            .filter((c) => c.id !== undefined)
            .map((c) => ({
                id: c.id!,
                displayLabel: c.name,
            })),
    );

    const personItems = $derived(
        $people
            .filter((p) => p.id !== undefined)
            .map((p) => ({
                id: p.id!,
                displayLabel: `${p.firstName} ${p.lastName}`,
            })),
    );

    const jobListingItems = $derived(
        $jobListings
            .filter((j) => j.id !== undefined)
            .map((j) => ({
                id: j.id!,
                displayLabel: j.title,
            })),
    );

    const applicationItems = $derived(
        $applications
            .filter((a) => a.id !== undefined)
            .map((a) => ({
                id: a.id!,
                displayLabel: `Application #${a.id}`,
            })),
    );

    const interactionItems = $derived(
        $interactions
            .filter((i) => i.id !== undefined)
            .map((i) => ({
                id: i.id!,
                displayLabel: `Interaction #${i.id}`,
            })),
    );

    let form = $state<Note>({
        interactionId: undefined,
        jobListingId: undefined,
        applicationId: undefined,
        personId: undefined,
        companyId: undefined,
        noteType: NoteType.General,
        title: "",
        content: "",
    });

    $effect(() => {
        if (!noteID) return;
        const found = $notes.find((n) => n.id === noteID);
        if (found) Object.assign(form, found);
    });

    const invalidSubmit = $derived(!form.title || !form.content);

    async function submit() {
        if (invalidSubmit) return;

        let createdNote: Note;

        if (noteID) {
            await updateNote(Number(noteID), form);
            createdNote = form;
        } else {
            createdNote = await createNote(form);
        }

        console.log($state.snapshot(form));
        if (caller && !noteID) {
            newlyCreatedNoteId.set(createdNote.id);
        }

        if (caller) {
            goto(`/${caller}/create`);
        } else {
            goto("/notes");
        }
    }
</script>

<section class="w-1/3 mx-auto flex flex-col justify-center gap-4">
    <h1 class="text-xl font-extrabold">
        {noteID ? "Edit Note" : "Create a new Note"}
    </h1>

    <div class="space-y-4">
        <div>
            <Label for="company" class="py-2">Company</Label>
            <CustomIDSelectCreate
                items={companyItems}
                bind:value={form.companyId}
                caller="notes"
                createNew="companies"
            />
        </div>

        <div>
            <Label for="person" class="py-2">Person</Label>
            <CustomIDSelectCreate
                items={personItems}
                bind:value={form.personId}
                caller="notes"
                createNew="people"
            />
        </div>

        <div>
            <Label for="jobListing" class="py-2">Job Listing</Label>
            <CustomIDSelectCreate
                items={jobListingItems}
                bind:value={form.jobListingId}
                caller="notes"
                createNew="jobListings"
            />
        </div>

        <div>
            <Label for="application" class="py-2">Application</Label>
            <CustomIDSelectCreate
                items={applicationItems}
                bind:value={form.applicationId}
                caller="notes"
                createNew="applications"
            />
        </div>

        <div>
            <Label for="interaction" class="py-2">Interaction</Label>
            <CustomIDSelectCreate
                items={interactionItems}
                bind:value={form.interactionId}
                caller="notes"
                createNew="interactions"
            />
        </div>

        <div>
            <Label for="noteType" class="py-2">Note Type</Label>
            <CustomEnumSelector
                enumObject={NoteType}
                bind:selectedValue={form.noteType}
            />
        </div>

        <div>
            <Label for="title" class="py-2 required">Title</Label>
            <Input
                id="title"
                bind:value={form.title}
                placeholder="Note title..."
            />
        </div>

        <div>
            <Label for="content" class="py-2 required">Content</Label>
            <Textarea
                id="content"
                bind:value={form.content}
                placeholder="Note content..."
                rows={6}
            />
        </div>
    </div>

    <div class="flex justify-between mt-6">
        <Button
            variant="destructive"
            href={caller ? `/${caller}/create?` : "/notes"}
        >
            Cancel
        </Button>
        <Button disabled={invalidSubmit} onclick={submit}>
            {noteID ? "Update" : "Create"}
        </Button>
    </div>
</section>
