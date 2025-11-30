<script lang="ts">
    import { goto } from "$app/navigation";
    import { Button } from "$lib/components/ui/button";
    import { Switch } from "$lib/components/ui/switch";
    import { notes } from "$lib/stores/notes";
    import { companies } from "$lib/stores/companies";
    import { people } from "$lib/stores/people";
    import { jobListings } from "$lib/stores/jobListings";
    import { applications } from "$lib/stores/applications";
    import {
        createReminder,
        reminders,
        updateReminder,
    } from "$lib/stores/reminders";
    import type { Reminder } from "$lib/types/reminder";
    import CustomIDSelectCreate from "./utils/CustomIDSelectCreate.svelte";
    import CustomDatePicker from "./utils/CustomDatePicker.svelte";
    import { Input } from "../ui/input";
    import { Label } from "../ui/label";
    import { Textarea } from "../ui/textarea";
    import {
        newlyCreatedReminderId,
        newlyCreatedNoteId,
        newlyCreatedApplicationId,
        newlyCreatedPersonId,
        newlyCreatedCompanyId,
        newlyCreatedJobListingId,
    } from "$lib/stores/formState";
    import { onMount } from "svelte";

    let {
        reminderID,
        callerChain = [],
    }: { reminderID?: number; callerChain?: string[] } = $props();

    const noteItems = $derived(
        $notes
            .filter((n) => n.id !== undefined)
            .map((n) => ({
                id: n.id!,
                displayLabel: n.title || `Note #${n.id}`,
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

    // Filter job listings by selected company
    const jobListingItems = $derived.by(() => {
        if (companyId === undefined) {
            return [];
        }
        return $jobListings
            .filter((j) => j.id !== undefined && j.companyId === companyId)
            .map((j) => ({
                id: j.id!,
                displayLabel: j.title || `Job Listing #${j.id}`,
            }));
    });

    const personItems = $derived(
        $people
            .filter((p) => p.id !== undefined)
            .map((p) => ({
                id: p.id!,
                displayLabel: `${p.firstName} ${p.lastName}`,
            })),
    );

    const companyItems = $derived(
        $companies
            .filter((c) => c.id !== undefined)
            .map((c) => ({
                id: c.id!,
                displayLabel: c.name || `Company #${c.id}`,
            })),
    );

    let form = $state<Reminder>({
        title: "",
        message: "",
        reminderDate: new Date().toISOString().split("T")[0],
        isCompleted: false,
        noteId: undefined,
        jobListingId: undefined,
        applicationId: undefined,
        personId: undefined,
        companyId: undefined,
    });

    // Separate state variables for binding
    let noteId = $state<number | undefined>(undefined);
    let applicationId = $state<number | undefined>(undefined);
    let jobListingId = $state<number | undefined>(undefined);
    let personId = $state<number | undefined>(undefined);
    let companyId = $state<number | undefined>(undefined);
    let reminderDate = $state<string>(new Date().toISOString().split("T")[0]);

    // Sync state variables to form
    $effect(() => {
        form.noteId = noteId;
    });

    $effect(() => {
        form.applicationId = applicationId;
    });

    $effect(() => {
        form.jobListingId = jobListingId;
    });

    $effect(() => {
        form.personId = personId;
    });

    $effect(() => {
        form.companyId = companyId;
    });

    $effect(() => {
        form.reminderDate = reminderDate;
    });

    // Clear job listing when company changes
    $effect(() => {
        if (companyId === undefined) {
            jobListingId = undefined;
        }
    });

    $effect(() => {
        if (!reminderID) return;
        const found = $reminders.find((r) => r.id === reminderID);
        if (found) {
            Object.assign(form, found);
            // Sync to state variables too
            if (found.noteId) noteId = found.noteId;
            if (found.applicationId) applicationId = found.applicationId;
            if (found.jobListingId) jobListingId = found.jobListingId;
            if (found.personId) personId = found.personId;
            if (found.companyId) companyId = found.companyId;
            reminderDate = found.reminderDate;
        }
    });

    const invalidSubmit = $derived(!form.title || !form.message);

    // Listen for newly created items when returning from nested forms
    onMount(() => {
        const unsubscribers: (() => void)[] = [];

        // Subscribe to note store
        unsubscribers.push(
            newlyCreatedNoteId.subscribe((id) => {
                if (id !== undefined) {
                    noteId = id;
                    setTimeout(() => newlyCreatedNoteId.set(undefined), 0);
                }
            }),
        );

        // Subscribe to application store
        unsubscribers.push(
            newlyCreatedApplicationId.subscribe((id) => {
                if (id !== undefined) {
                    applicationId = id;
                    setTimeout(
                        () => newlyCreatedApplicationId.set(undefined),
                        0,
                    );
                }
            }),
        );

        // Subscribe to job listing store - extract company from job listing
        unsubscribers.push(
            newlyCreatedJobListingId.subscribe((id) => {
                if (id !== undefined) {
                    // Find the job listing to get its company ID
                    const jobListing = $jobListings.find((j) => j.id === id);
                    if (jobListing && jobListing.companyId) {
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

        // Subscribe to person store
        unsubscribers.push(
            newlyCreatedPersonId.subscribe((id) => {
                if (id !== undefined) {
                    personId = id;
                    setTimeout(() => newlyCreatedPersonId.set(undefined), 0);
                }
            }),
        );

        // Subscribe to company store
        unsubscribers.push(
            newlyCreatedCompanyId.subscribe((id) => {
                if (id !== undefined) {
                    companyId = id;
                    jobListingId = undefined; // Clear job listing when company changes
                    setTimeout(() => newlyCreatedCompanyId.set(undefined), 0);
                }
            }),
        );

        return () => {
            unsubscribers.forEach((unsub) => unsub());
        };
    });

    async function submit() {
        if (invalidSubmit) return;

        let createdReminder: Reminder;

        if (reminderID) {
            await updateReminder(Number(reminderID), form);
            createdReminder = form;
        } else {
            createdReminder = await createReminder(form);
        }

        if (callerChain.length > 0 && !reminderID) {
            newlyCreatedReminderId.set(createdReminder.id);
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
            await goto("/reminders");
        }
    }

    function handleCancel() {
        if (callerChain.length > 0) {
            const url = `/${callerChain[0]}/create`;
            goto(url);
        } else {
            goto("/reminders");
        }
    }
</script>

<section class="w-1/3 mx-auto flex flex-col justify-center gap-4">
    <h1 class="text-xl font-extrabold">
        {reminderID ? "Edit Reminder" : "Create a new Reminder"}
    </h1>

    <div class="space-y-4">
        <!-- Title -->
        <div>
            <Label for="title" class="py-2 required">Title</Label>
            <Input
                id="title"
                bind:value={form.title}
                placeholder="Reminder title"
            />
        </div>

        <!-- Date & Status Row -->
        <div class="grid grid-cols-2 gap-4">
            <div>
                <Label for="reminderDate" class="py-2 required">Date</Label>
                <CustomDatePicker bind:selectedDate={reminderDate} />
            </div>
            <div>
                <Label class="py-2">Status</Label>
                <div
                    class="flex items-center h-10 px-3 border border-input rounded-md bg-background"
                >
                    <span class="text-sm text-muted-foreground">
                        {form.isCompleted ? "Done" : "Pending"}
                    </span>
                    <Switch bind:checked={form.isCompleted} />
                </div>
            </div>
        </div>

        <!-- Company -->
        <div>
            <Label for="company" class="py-2">Company</Label>
            <CustomIDSelectCreate
                items={companyItems}
                bind:value={companyId}
                {callerChain}
                currentPage="reminders"
                createNew="companies"
            />
        </div>

        <!-- Job Listing -->
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
                currentPage="reminders"
                createNew="jobListings"
                disabled={companyId === undefined}
            />
        </div>

        <!-- Application -->
        <div>
            <Label for="application" class="py-2">Application</Label>
            <CustomIDSelectCreate
                items={applicationItems}
                bind:value={applicationId}
                {callerChain}
                currentPage="reminders"
                createNew="applications"
            />
        </div>

        <!-- Person -->
        <div>
            <Label for="person" class="py-2">Person</Label>
            <CustomIDSelectCreate
                items={personItems}
                bind:value={personId}
                {callerChain}
                currentPage="reminders"
                createNew="people"
            />
        </div>

        <!-- Note -->
        <div>
            <Label for="note" class="py-2">Note</Label>
            <CustomIDSelectCreate
                items={noteItems}
                bind:value={noteId}
                {callerChain}
                currentPage="reminders"
                createNew="notes"
            />
        </div>

        <!-- Message -->
        <div>
            <Label for="message" class="py-2 required">Message</Label>
            <Textarea
                id="message"
                bind:value={form.message}
                placeholder="Reminder details..."
                rows={6}
            />
        </div>
    </div>

    <div class="flex justify-between mt-6">
        <Button variant="destructive" onclick={handleCancel}>Cancel</Button>
        <Button disabled={invalidSubmit} onclick={submit}>
            {reminderID ? "Update" : "Create"}
        </Button>
    </div>
</section>
