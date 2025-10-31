<script lang="ts">
    import * as Dialog from "$lib/components/ui/dialog";
    import { Button } from "$lib/components/ui/button";
    import { Label } from "$lib/components/ui/label";
    import { Textarea } from "$lib/components/ui/textarea";
    import CustomAlert from "./utils/CustomAlert.svelte";
    import CustomDatePicker from "./utils/CustomDatePicker.svelte";
    import CustomEnumSelector from "./utils/CustomEnumSelector.svelte";
    import CustomIDSelector from "./utils/CustomIDSelector.svelte";
    import { jobListings } from "$lib/stores/jobListings";
    import {
        createApplication,
        updateApplication,
    } from "$lib/stores/applications";
    import { createDefaultForm } from "./utils/baseForm.svelte";
    import { parseDate, getLocalTimeZone } from "@internationalized/date";
    import { Stage } from "$lib/types/enums";
    import type { Application } from "$lib/types/application";

    let {
        open = $bindable(false),
        mode = "create",
        existingApplication = null as Application | null,
    } = $props();

    const defaultForm = {
        jobListingId: null as number | null,
        stage: Stage.Applied,
        appliedDate: parseDate(new Date().toISOString().split("T")[0]),
        applicationNotes: "",
    };
    const { formData, resetForm } = createDefaultForm(defaultForm);

    $effect(() => {
        if (mode === "edit" && existingApplication) {
            $formData = {
                jobListingId: existingApplication.jobListingId ?? null,
                stage: existingApplication.stage,
                appliedDate: parseDate(existingApplication.appliedDate),
                applicationNotes: existingApplication.applicationNotes ?? "",
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
        const appliedDate = fd.appliedDate
            ? `${fd.appliedDate.year}-${String(fd.appliedDate.month).padStart(2, "0")}-${String(fd.appliedDate.day).padStart(2, "0")}`
            : new Date().toISOString().split("T")[0];

        try {
            if (mode === "create") {
                await createApplication({
                    jobListingId: fd.jobListingId ?? undefined,
                    stage: fd.stage,
                    appliedDate,
                    applicationNotes: fd.applicationNotes,
                    displayLabel: `Application ${fd.jobListingId ?? "New"}`,
                });
                showAlert(
                    "success",
                    "Application Created",
                    "New application was added.",
                );
            } else if (mode === "edit" && existingApplication) {
                await updateApplication(existingApplication.id, {
                    jobListingId: fd.jobListingId ?? undefined,
                    stage: fd.stage,
                    appliedDate,
                    applicationNotes: fd.applicationNotes,
                });
                showAlert(
                    "success",
                    "Application Updated",
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
                >{mode === "edit"
                    ? "Edit Application"
                    : "Create Application"}</Dialog.Title
            >
            <Dialog.Description class="text-sm text-muted-foreground">
                {mode === "edit"
                    ? "Modify application details below."
                    : "Provide details to add a new application."}
            </Dialog.Description>
        </Dialog.Header>

        <form class="flex flex-col gap-5" onsubmit={handleSubmit}>
            <div class="grid sm:grid-cols-2 gap-4">
                <div>
                    <Label>Job Listing</Label><CustomIDSelector
                        sourceStore={jobListings}
                        bind:selectedId={$formData.jobListingId}
                    />
                </div>
                <div>
                    <Label>Stage</Label><CustomEnumSelector
                        enumObject={Stage}
                        bind:selectedValue={$formData.stage}
                    />
                </div>
            </div>

            <div>
                <Label>Date Applied</Label><CustomDatePicker
                    bind:selectedDate={$formData.appliedDate}
                />
            </div>

            <div>
                <Label>Notes</Label><Textarea
                    bind:value={$formData.applicationNotes}
                    placeholder="Add notes or context..."
                />
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
