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
    import CustomEnumSelector from "./utils/CustomEnumSelector.svelte";
    import CustomIDSelectCreate from "./utils/CustomIDSelectCreate.svelte";
    import { Input } from "../ui/input";
    import { Label } from "../ui/label";
    import { Textarea } from "../ui/textarea";
    import {
        newlyCreatedNoteId,
        newlyCreatedCompanyId,
        newlyCreatedPersonId,
        newlyCreatedJobListingId,
        newlyCreatedApplicationId,
        newlyCreatedInteractionId,
    } from "$lib/stores/formState";
    import { onMount } from "svelte";

    let {
        noteID,
        callerChain = [],
    }: { noteID?: number; callerChain?: string[] } = $props();

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

    // Filter job listings by selected company
    const jobListingItems = $derived.by(() => {
        if (companyId === undefined) {
            return [];
        }
        return $jobListings
            .filter((j) => j.id !== undefined && j.companyId === companyId)
            .map((j) => ({
                id: j.id!,
                displayLabel: j.title,
            }));
    });

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

    // Separate state variables for binding (like JoblistingForm)
    let companyId = $state<number | undefined>(undefined);
    let personId = $state<number | undefined>(undefined);
    let jobListingId = $state<number | undefined>(undefined);
    let applicationId = $state<number | undefined>(undefined);
    let interactionId = $state<number | undefined>(undefined);

    // Sync state variables to form
    $effect(() => {
        form.companyId = companyId;
    });

    $effect(() => {
        form.personId = personId;
    });

    $effect(() => {
        form.jobListingId = jobListingId;
    });

    $effect(() => {
        form.applicationId = applicationId;
    });

    $effect(() => {
        form.interactionId = interactionId;
    });

    // Clear job listing when company changes
    $effect(() => {
        if (companyId === undefined) {
            jobListingId = undefined;
        }
    });

    $effect(() => {
        if (!noteID) return;
        const found = $notes.find((n) => n.id === noteID);
        if (found) {
            Object.assign(form, found);
            // Sync to state variables too
            if (found.companyId) companyId = found.companyId;
            if (found.personId) personId = found.personId;
            if (found.jobListingId) jobListingId = found.jobListingId;
            if (found.applicationId) applicationId = found.applicationId;
            if (found.interactionId) interactionId = found.interactionId;
        }
    });

    const invalidSubmit = $derived(!form.title || !form.content);

    // Listen for newly created items when returning from nested forms
    onMount(() => {
        const unsubscribers: (() => void)[] = [];

        // Subscribe to company store
        unsubscribers.push(
            newlyCreatedCompanyId.subscribe((id) => {
                if (id !== undefined) {
                    console.log("Auto-selecting newly created company:", id);
                    companyId = id;
                    jobListingId = undefined; // Clear job listing when company changes
                    setTimeout(() => newlyCreatedCompanyId.set(undefined), 0);
                }
            }),
        );

        // Subscribe to person store
        unsubscribers.push(
            newlyCreatedPersonId.subscribe((id) => {
                if (id !== undefined) {
                    console.log("Auto-selecting newly created person:", id);
                    personId = id;
                    setTimeout(() => newlyCreatedPersonId.set(undefined), 0);
                }
            }),
        );

        // Subscribe to job listing store - extract company from job listing
        unsubscribers.push(
            newlyCreatedJobListingId.subscribe((id) => {
                if (id !== undefined) {
                    console.log(
                        "Auto-selecting newly created job listing:",
                        id,
                    );
                    // Find the job listing to get its company ID
                    const jobListing = $jobListings.find((j) => j.id === id);
                    if (jobListing && jobListing.companyId) {
                        console.log(
                            "Setting company from job listing:",
                            jobListing.companyId,
                        );
                        companyId = jobListing.companyId;
                        // Now set the job listing after company is set
                        setTimeout(() => {
                            jobListingId = id;
                            newlyCreatedJobListingId.set(undefined);
                        }, 0);
                    } else {
                        jobListingId = id;
                        setTimeout(
                            () => newlyCreatedJobListingId.set(undefined),
                            0,
                        );
                    }
                }
            }),
        );

        // Subscribe to application store
        unsubscribers.push(
            newlyCreatedApplicationId.subscribe((id) => {
                if (id !== undefined) {
                    console.log(
                        "Auto-selecting newly created application:",
                        id,
                    );
                    applicationId = id;
                    setTimeout(
                        () => newlyCreatedApplicationId.set(undefined),
                        0,
                    );
                }
            }),
        );

        // Subscribe to interaction store
        unsubscribers.push(
            newlyCreatedInteractionId.subscribe((id) => {
                if (id !== undefined) {
                    console.log(
                        "Auto-selecting newly created interaction:",
                        id,
                    );
                    interactionId = id;
                    setTimeout(
                        () => newlyCreatedInteractionId.set(undefined),
                        0,
                    );
                }
            }),
        );

        return () => {
            unsubscribers.forEach((unsub) => unsub());
        };
    });

    async function submit() {
        if (invalidSubmit) return;

        let createdNote: Note;

        if (noteID) {
            await updateNote(Number(noteID), form);
            createdNote = form;
        } else {
            createdNote = await createNote(form);
        }

        if (callerChain.length > 0 && !noteID) {
            newlyCreatedNoteId.set(createdNote.id);
        }

        // Go back to where we came from
        if (callerChain.length > 0) {
            // Get the page we came from (last item in chain)
            const backToCaller = callerChain[callerChain.length - 1];
            // Build new chain WITHOUT the current page (remove last item)
            const newChain = callerChain.slice(0, -1);

            if (newChain.length > 0) {
                // Pass remaining chain to previous page
                const params = new URLSearchParams();
                params.set("callerChain", newChain.join(","));
                const url = `/${backToCaller}/create? ${params.toString()}`;
                await goto(url);
            } else {
                // No more chain, just go back to caller
                const url = `/${backToCaller}/create`;
                await goto(url);
            }
        } else {
            await goto("/notes");
        }
    }

    function handleCancel() {
        if (callerChain.length > 0) {
            const url = `/${callerChain[0]}/create`;
            goto(url);
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
                bind:value={companyId}
                {callerChain}
                currentPage="notes"
                createNew="companies"
            />
        </div>

        <div>
            <Label for="person" class="py-2">Person</Label>
            <CustomIDSelectCreate
                items={personItems}
                bind:value={personId}
                {callerChain}
                currentPage="notes"
                createNew="people"
            />
        </div>

        <div>
            <Label for="jobListing" class="py-2">
                Job Listing
                {#if companyId === undefined}
                    <span class="text-sm text-gray-500"
                        >(Select a company first)</span
                    >
                {/if}
            </Label>
            <CustomIDSelectCreate
                items={jobListingItems}
                bind:value={jobListingId}
                {callerChain}
                currentPage="notes"
                createNew="jobListings"
                disabled={companyId === undefined}
            />
        </div>

        <div>
            <Label for="application" class="py-2">Application</Label>
            <CustomIDSelectCreate
                items={applicationItems}
                bind:value={applicationId}
                {callerChain}
                currentPage="notes"
                createNew="applications"
            />
        </div>

        <div>
            <Label for="interaction" class="py-2">Interaction</Label>
            <CustomIDSelectCreate
                items={interactionItems}
                bind:value={interactionId}
                {callerChain}
                currentPage="notes"
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
        <Button variant="destructive" onclick={handleCancel}>Cancel</Button>
        <Button disabled={invalidSubmit} onclick={submit}>
            {noteID ? "Update" : "Create"}
        </Button>
    </div>
</section>
