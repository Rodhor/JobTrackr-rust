<script lang="ts">
    import { onMount } from "svelte";
    import {
        applications,
        loadApplications,
        createApplication,
        deleteApplication,
    } from "$lib/stores/applications";
    import * as Dialog from "$lib/components/ui/dialog/index.js";
    import { Button } from "$lib/components/ui/button/index.js";

    // Types
    import type { Application } from "$lib/types/application";
    import type { Status } from "$lib/types/enums"; // adjust if your enums live elsewhere

    // Modal state (local form model)
    let open = false;

    // Minimal fields required by your backend for create
    let user_id: number = 1; // use an existing user id in your DB
    let job_listing_id: number = 1; // use an existing job_listing id
    let statusValue: Status = "applied";
    let applied_date: string = new Date().toISOString(); // ISO string (matches TEXT/UTC)
    let cv_file_path: string = "";
    let cover_letter_file_path: string = "";
    let application_notes: string = "";

    onMount(() => {
        loadApplications();
    });

    function resetForm() {
        user_id = 1;
        job_listing_id = 1;
        statusValue = "applied";
        applied_date = new Date().toISOString();
        cv_file_path = "";
        cover_letter_file_path = "";
        application_notes = "";
    }

    async function handleCreate(e: Event) {
        e.preventDefault();

        // Build payload matching Omit<Application, "id" | "created_at" | "updated_at">
        const payload = {
            userId: Number(user_id),
            jobListingId: Number(job_listing_id),
            status: statusValue,
            appliedDate: applied_date || undefined,
            cvFilePath: cv_file_path || undefined,
            coverLetterFilePath: cover_letter_file_path || undefined,
            applicationNotes: application_notes || undefined,
        } as const;

        await createApplication(payload as any); // store expects that Omit<Application,...> shape
        open = false;
        resetForm();
    }

    async function handleDelete(id: number) {
        await deleteApplication(id);
    }
</script>

<!-- Header + Create button -->
<div class="mb-4 flex items-center justify-between">
    <h1 class="text-xl font-semibold">Applications</h1>

    <Dialog.Root bind:open>
        <Dialog.Trigger asChild>
            <Button variant="default">New Application</Button>
        </Dialog.Trigger>

        <Dialog.Content class="max-w-xl">
            <Dialog.Header>
                <Dialog.Title>Create Application</Dialog.Title>
                <Dialog.Description
                    >Quick test modal to create a new application.</Dialog.Description
                >
            </Dialog.Header>

            <form class="space-y-4" on:submit|preventDefault={handleCreate}>
                <div class="grid grid-cols-2 gap-3">
                    <label class="flex flex-col gap-1">
                        <span class="text-sm">User ID</span>
                        <input
                            type="number"
                            class="rounded-md border bg-background px-3 py-2"
                            bind:value={user_id}
                            min="1"
                            required
                        />
                    </label>

                    <label class="flex flex-col gap-1">
                        <span class="text-sm">Job Listing ID</span>
                        <input
                            type="number"
                            class="rounded-md border bg-background px-3 py-2"
                            bind:value={job_listing_id}
                            min="1"
                            required
                        />
                    </label>

                    <label class="flex flex-col gap-1">
                        <span class="text-sm">Status</span>
                        <select
                            class="rounded-md border bg-background px-3 py-2"
                            bind:value={statusValue}
                            required
                        >
                            <option value="applied">applied</option>
                            <option value="interviewing">interviewing</option>
                            <option value="offered">offered</option>
                            <option value="rejected">rejected</option>
                            <option value="withdrawn">withdrawn</option>
                            <option value="other">other</option>
                        </select>
                    </label>

                    <label class="flex flex-col gap-1">
                        <span class="text-sm">Applied Date (ISO)</span>
                        <input
                            type="text"
                            class="rounded-md border bg-background px-3 py-2"
                            bind:value={applied_date}
                            required
                        />
                    </label>
                </div>

                <label class="flex flex-col gap-1">
                    <span class="text-sm">CV File Path</span>
                    <input
                        type="text"
                        class="rounded-md border bg-background px-3 py-2"
                        bind:value={cv_file_path}
                        placeholder="/home/user/Documents/cv.pdf"
                    />
                </label>

                <label class="flex flex-col gap-1">
                    <span class="text-sm">Cover Letter File Path</span>
                    <input
                        type="text"
                        class="rounded-md border bg-background px-3 py-2"
                        bind:value={cover_letter_file_path}
                        placeholder="/home/user/Documents/cover_letter.pdf"
                    />
                </label>

                <label class="flex flex-col gap-1">
                    <span class="text-sm">Notes</span>
                    <textarea
                        class="min-h-24 rounded-md border bg-background px-3 py-2"
                        bind:value={application_notes}
                        placeholder="Notes about this application…"
                    />
                </label>

                <Dialog.Footer class="flex justify-end gap-2">
                    <Dialog.Close asChild>
                        <Button type="button" variant="secondary">Cancel</Button
                        >
                    </Dialog.Close>
                    <Button type="submit" variant="default">Create</Button>
                </Dialog.Footer>
            </form>
        </Dialog.Content>
    </Dialog.Root>
</div>

<!-- List -->
<div class="overflow-x-auto rounded-lg border">
    <table class="min-w-full text-sm">
        <thead class="bg-muted/50 text-left">
            <tr>
                <th class="px-4 py-2">ID</th>
                <th class="px-4 py-2">User</th>
                <th class="px-4 py-2">Job Listing</th>
                <th class="px-4 py-2">Status</th>
                <th class="px-4 py-2">Applied</th>
                <th class="px-4 py-2">Actions</th>
            </tr>
        </thead>
        <tbody>
            {#each $applications as a (a.id)}
                <tr class="border-t">
                    <td class="px-4 py-2">{a.id}</td>
                    <td class="px-4 py-2">{a.user_id}</td>
                    <td class="px-4 py-2">{a.job_listing_id}</td>
                    <td class="px-4 py-2">{a.status}</td>
                    <td class="px-4 py-2">{a.applied_date}</td>
                    <td class="px-4 py-2">
                        <button
                            class="rounded-md border px-2 py-1 hover:bg-destructive hover:text-destructive-foreground"
                            on:click={() => handleDelete(a.id)}
                        >
                            Delete
                        </button>
                    </td>
                </tr>
            {/each}
            {#if $applications.length === 0}
                <tr class="border-t">
                    <td
                        class="px-4 py-8 text-center text-muted-foreground"
                        colspan="6"
                    >
                        No applications yet.
                    </td>
                </tr>
            {/if}
        </tbody>
    </table>
</div>
