<script lang="ts">
    import * as Dialog from "$lib/components/ui/dialog";
    import { Button } from "$lib/components/ui/button";
    import { Label } from "$lib/components/ui/label";
    import { Input } from "$lib/components/ui/input";
    import { Textarea } from "$lib/components/ui/textarea";
    import CustomAlert from "./utils/CustomAlert.svelte";
    import CustomEnumSelector from "./utils/CustomEnumSelector.svelte";
    import CustomIDSelector from "./utils/CustomIDSelector.svelte";
    import { createNote, updateNote } from "$lib/stores/notes";
    import { applications } from "$lib/stores/applications";
    import { people } from "$lib/stores/people";
    import { companies } from "$lib/stores/companies";
    import { jobListings } from "$lib/stores/jobListings";
    import { interactions } from "$lib/stores/interactions";
    import { createDefaultForm } from "./utils/baseForm.svelte";
    import { NoteType } from "$lib/types/enums";
    import type { Note } from "$lib/types/note";

    let {
        open = $bindable(false),
        mode = "create",
        existingNote = null as Note | null,
    } = $props();

    const defaultForm = {
        interactionId: null as number | null,
        jobListingId: null as number | null,
        applicationId: null as number | null,
        personId: null as number | null,
        companyId: null as number | null,
        noteType: NoteType.General,
        title: "",
        content: "",
    };
    const { formData, resetForm } = createDefaultForm(defaultForm);

    $effect(() => {
        if (mode === "edit" && existingNote) {
            $formData = {
                interactionId: existingNote.interactionId ?? null,
                jobListingId: existingNote.jobListingId ?? null,
                applicationId: existingNote.applicationId ?? null,
                personId: existingNote.personId ?? null,
                companyId: existingNote.companyId ?? null,
                noteType: existingNote.noteType ?? NoteType.General,
                title: existingNote.title ?? "",
                content: existingNote.content ?? "",
            };
        } else {
            resetForm();
        }
    });

    let alertVisible = $state(false);
    let alertStatus = $state<"success" | "error" | "info">("info");
    let alertMessage = $state("");
    let alertDescription = $state("");
    function showAlert(s: "success" | "error" | "info", m: string, d = "") {
        alertStatus = s;
        alertMessage = m;
        alertDescription = d;
        alertVisible = true;
        setTimeout(() => (alertVisible = false), 4000);
    }

    async function handleSubmit(e: SubmitEvent) {
        e.preventDefault();
        const fd = $formData;

        try {
            if (mode === "create") {
                await createNote({
                    noteType: fd.noteType,
                    title: fd.title,
                    content: fd.content,
                    companyId: fd.companyId ?? undefined,
                    personId: fd.personId ?? undefined,
                    applicationId: fd.applicationId ?? undefined,
                    jobListingId: fd.jobListingId ?? undefined,
                    interactionId: fd.interactionId ?? undefined,
                    displayLabel: fd.title?.trim() || `Note (${fd.noteType})`,
                });
                showAlert("success", "Note Created", "New note was added.");
            } else if (mode === "edit" && existingNote) {
                await updateNote(existingNote.id, {
                    noteType: fd.noteType,
                    title: fd.title,
                    content: fd.content,
                    companyId: fd.companyId ?? undefined,
                    personId: fd.personId ?? undefined,
                    applicationId: fd.applicationId ?? undefined,
                    jobListingId: fd.jobListingId ?? undefined,
                    interactionId: fd.interactionId ?? undefined,
                    displayLabel:
                        fd.title?.trim() ||
                        existingNote.displayLabel ||
                        `Note (${fd.noteType})`,
                });
                showAlert(
                    "success",
                    "Note Updated",
                    "Changes saved successfully.",
                );
            }
            resetForm();
            setTimeout(() => (open = false), 100);
        } catch (err: any) {
            console.error(err);
            showAlert(
                "error",
                "Operation Failed",
                err?.message || "Unexpected error.",
            );
        }
    }
</script>

{#if alertVisible}
    <div class="fixed top-6 left-1/2 z-50 -translate-x-1/2">
        <CustomAlert
            status={alertStatus}
            message={alertMessage}
            description={alertDescription}
        />
    </div>
{/if}

<Dialog.Root bind:open>
    <Dialog.Content class="!max-w-2xl">
        <Dialog.Header>
            <Dialog.Title class="text-lg font-semibold"
                >{mode === "edit" ? "Edit Note" : "Create Note"}</Dialog.Title
            >
            <Dialog.Description class="text-sm text-muted-foreground">
                {mode === "edit"
                    ? "Modify note details below."
                    : "Provide details to add a new note."}
            </Dialog.Description>
        </Dialog.Header>

        <form class="flex flex-col gap-5" onsubmit={handleSubmit}>
            <div class="grid sm:grid-cols-2 gap-4">
                <div>
                    <Label>Note Type</Label><CustomEnumSelector
                        enumObject={NoteType}
                        bind:selectedValue={$formData.noteType}
                    />
                </div>
                <div>
                    <Label>Title</Label><Input
                        bind:value={$formData.title}
                        placeholder="Note title"
                    />
                </div>
            </div>

            <div>
                <Label>Content</Label><Textarea
                    bind:value={$formData.content}
                    placeholder="Write your note here..."
                />
            </div>

            <div class="grid sm:grid-cols-3 gap-4">
                <div>
                    <Label>Company</Label><CustomIDSelector
                        sourceStore={companies}
                        bind:selectedId={$formData.companyId}
                    />
                </div>
                <div>
                    <Label>Person</Label><CustomIDSelector
                        sourceStore={people}
                        bind:selectedId={$formData.personId}
                    />
                </div>
                <div>
                    <Label>Application</Label><CustomIDSelector
                        sourceStore={applications}
                        bind:selectedId={$formData.applicationId}
                    />
                </div>
            </div>

            <div class="grid sm:grid-cols-2 gap-4">
                <div>
                    <Label>Job Listing</Label><CustomIDSelector
                        sourceStore={jobListings}
                        bind:selectedId={$formData.jobListingId}
                    />
                </div>
                <div>
                    <Label>Interaction</Label><CustomIDSelector
                        sourceStore={interactions}
                        bind:selectedId={$formData.interactionId}
                    />
                </div>
            </div>

            <Dialog.Footer class="mt-2 flex justify-end gap-3 border-t pt-3">
                <Button
                    type="button"
                    variant="secondary"
                    onclick={() => {
                        resetForm();
                        open = false;
                    }}>Cancel</Button
                >
                <Button type="submit" variant="default"
                    >{mode === "edit" ? "Save Changes" : "Create"}</Button
                >
            </Dialog.Footer>
        </form>
    </Dialog.Content>
</Dialog.Root>
