<script lang="ts">
    import * as Dialog from "$lib/components/ui/dialog";
    import { Button } from "$lib/components/ui/button";
    import { Label } from "$lib/components/ui/label";
    import { Input } from "$lib/components/ui/input";
    import { Textarea } from "$lib/components/ui/textarea";
    import CustomAlert from "./utils/CustomAlert.svelte";
    import CustomDatePicker from "./utils/CustomDatePicker.svelte";
    import CustomIDSelector from "./utils/CustomIDSelector.svelte";
    import { createDefaultForm } from "./utils/baseForm.svelte";
    import { createReminder, updateReminder } from "$lib/stores/reminders";
    import { parseDate, getLocalTimeZone } from "@internationalized/date";
    import { notes } from "$lib/stores/notes";
    import { companies } from "$lib/stores/companies";
    import { people } from "$lib/stores/people";
    import { jobListings } from "$lib/stores/jobListings";
    import { applications } from "$lib/stores/applications";
    import type { Reminder } from "$lib/types/reminder";

    let {
        open = $bindable(false),
        mode = "create",
        existingReminder = null as Reminder | null,
    } = $props();

    const defaultForm = {
        title: "",
        message: "",
        reminderDate: parseDate(new Date().toISOString().split("T")[0]),
        isCompleted: false,
        noteId: null as number | null,
        jobListingId: null as number | null,
        applicationId: null as number | null,
        personId: null as number | null,
        companyId: null as number | null,
    };
    const { formData, resetForm } = createDefaultForm(defaultForm);

    $effect(() => {
        if (mode === "edit" && existingReminder) {
            $formData = {
                title: existingReminder.title ?? "",
                message: existingReminder.message ?? "",
                reminderDate: parseDate(existingReminder.reminderDate),
                isCompleted: existingReminder.isCompleted ?? false,
                noteId: existingReminder.noteId ?? null,
                jobListingId: existingReminder.jobListingId ?? null,
                applicationId: existingReminder.applicationId ?? null,
                personId: existingReminder.personId ?? null,
                companyId: existingReminder.companyId ?? null,
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
        const reminderDate = fd.reminderDate
            ? `${fd.reminderDate.year}-${String(fd.reminderDate.month).padStart(2, "0")}-${String(fd.reminderDate.day).padStart(2, "0")}`
            : new Date().toISOString().split("T")[0];

        try {
            if (mode === "create") {
                await createReminder({
                    title: fd.title,
                    message: fd.message,
                    reminderDate,
                    isCompleted: fd.isCompleted,
                    noteId: fd.noteId ?? undefined,
                    jobListingId: fd.jobListingId ?? undefined,
                    applicationId: fd.applicationId ?? undefined,
                    personId: fd.personId ?? undefined,
                    companyId: fd.companyId ?? undefined,
                    displayLabel:
                        fd.title?.trim() || `Reminder (${reminderDate})`,
                });
                showAlert(
                    "success",
                    "Reminder Created",
                    "A new reminder was added.",
                );
            } else if (mode === "edit" && existingReminder) {
                await updateReminder(existingReminder.id, {
                    title: fd.title,
                    message: fd.message,
                    reminderDate,
                    isCompleted: fd.isCompleted,
                    noteId: fd.noteId ?? undefined,
                    jobListingId: fd.jobListingId ?? undefined,
                    applicationId: fd.applicationId ?? undefined,
                    personId: fd.personId ?? undefined,
                    companyId: fd.companyId ?? undefined,
                    displayLabel:
                        fd.title?.trim() ||
                        existingReminder.displayLabel ||
                        `Reminder (${reminderDate})`,
                });
                showAlert(
                    "success",
                    "Reminder Updated",
                    "Changes saved successfully.",
                );
            }
            resetForm();
            setTimeout(() => (open = false), 100);
        } catch (err: any) {
            console.error(err);
            const idInfo =
                mode === "edit" && existingReminder
                    ? ` (ID: ${existingReminder.id})`
                    : "";
            showAlert(
                "error",
                `Operation Failed${idInfo}`,
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
                >{mode === "edit"
                    ? "Edit Reminder"
                    : "Create Reminder"}</Dialog.Title
            >
            <Dialog.Description class="text-sm text-muted-foreground">
                {mode === "edit"
                    ? "Modify reminder details below."
                    : "Provide details to create a new reminder."}
            </Dialog.Description>
        </Dialog.Header>

        <form class="flex flex-col gap-5" onsubmit={handleSubmit}>
            <div class="grid sm:grid-cols-2 gap-4">
                <div>
                    <Label>Title</Label><Input
                        bind:value={$formData.title}
                        placeholder="Reminder title"
                    />
                </div>
                <div>
                    <Label>Date</Label><CustomDatePicker
                        bind:selectedDate={$formData.reminderDate}
                    />
                </div>
            </div>

            <div>
                <Label>Message</Label><Textarea
                    bind:value={$formData.message}
                    placeholder="Reminder details..."
                />
            </div>

            <div class="grid sm:grid-cols-3 gap-4">
                <div>
                    <Label>Note</Label><CustomIDSelector
                        sourceStore={notes}
                        bind:selectedId={$formData.noteId}
                    />
                </div>
                <div>
                    <Label>Application</Label><CustomIDSelector
                        sourceStore={applications}
                        bind:selectedId={$formData.applicationId}
                    />
                </div>
                <div>
                    <Label>Job Listing</Label><CustomIDSelector
                        sourceStore={jobListings}
                        bind:selectedId={$formData.jobListingId}
                    />
                </div>
            </div>

            <div class="grid sm:grid-cols-2 gap-4">
                <div>
                    <Label>Person</Label><CustomIDSelector
                        sourceStore={people}
                        bind:selectedId={$formData.personId}
                    />
                </div>
                <div>
                    <Label>Company</Label><CustomIDSelector
                        sourceStore={companies}
                        bind:selectedId={$formData.companyId}
                    />
                </div>
            </div>

            <div class="flex items-center gap-2">
                <input
                    id="done"
                    type="checkbox"
                    class="rounded border-border text-primary focus:ring-0"
                    bind:checked={$formData.isCompleted}
                />
                <Label for="done">Mark as completed</Label>
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
