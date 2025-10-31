<script lang="ts">
    import * as Dialog from "$lib/components/ui/dialog";
    import { Button } from "$lib/components/ui/button";
    import { Label } from "$lib/components/ui/label";
    import { Input } from "$lib/components/ui/input";
    import { Textarea } from "$lib/components/ui/textarea";
    import CustomAlert from "./utils/CustomAlert.svelte";
    import CustomEnumSelector from "./utils/CustomEnumSelector.svelte";
    import CustomIDSelector from "./utils/CustomIDSelector.svelte";
    import {
        createJobListing,
        updateJobListing,
    } from "$lib/stores/jobListings";
    import { companies } from "$lib/stores/companies";
    import { createDefaultForm } from "./utils/baseForm.svelte";
    import {
        WorkType,
        WorkTypeDisplay,
        SeniorityLevel,
        SeniorityLevelDisplay,
        Currency,
        CurrencyDisplay,
    } from "$lib/types/enums";
    import type { JobListing } from "$lib/types/jobListing";

    let {
        open = $bindable(false),
        mode = "create",
        existingJobListing = null as JobListing | null,
    } = $props();

    const defaultForm = {
        companyId: null as number | null,
        title: "",
        workType: WorkType.Remote,
        category: "",
        seniorityLevel: SeniorityLevel.Mid,
        salaryMin: null as number | null,
        salaryMax: null as number | null,
        currency: Currency.EUR,
        description: "",
        url: "",
    };
    const { formData, resetForm } = createDefaultForm(defaultForm);

    $effect(() => {
        if (mode === "edit" && existingJobListing) {
            $formData = {
                companyId: existingJobListing.companyId ?? null,
                title: existingJobListing.title ?? "",
                workType: existingJobListing.workType ?? WorkType.Remote,
                category: existingJobListing.category ?? "",
                seniorityLevel:
                    existingJobListing.seniorityLevel ?? SeniorityLevel.Mid,
                salaryMin: existingJobListing.salaryMin ?? null,
                salaryMax: existingJobListing.salaryMax ?? null,
                currency: existingJobListing.currency ?? Currency.EUR,
                description: existingJobListing.description ?? "",
                url: existingJobListing.url ?? "",
            };
        } else {
            resetForm();
        }
    });

    let alertVisible = $state(false);
    let alertStatus = $state<"success" | "error" | "info">("info");
    let alertMessage = $state("");
    let alertDescription = $state("");
    function showAlert(
        status: "success" | "error" | "info",
        message: string,
        description = "",
    ) {
        alertStatus = status;
        alertMessage = message;
        alertDescription = description;
        alertVisible = true;
        setTimeout(() => (alertVisible = false), 4000);
    }

    async function handleSubmit(e: SubmitEvent) {
        e.preventDefault();
        const fd = $formData;
        if (fd.companyId === null) {
            showAlert(
                "error",
                "Missing Company",
                "Select a company before submitting.",
            );
            return;
        }

        try {
            if (mode === "create") {
                await createJobListing({
                    companyId: fd.companyId,
                    title: fd.title,
                    workType: fd.workType,
                    category: fd.category,
                    seniorityLevel: fd.seniorityLevel,
                    salaryMin: fd.salaryMin ?? undefined,
                    salaryMax: fd.salaryMax ?? undefined,
                    currency: fd.currency,
                    description: fd.description,
                    url: fd.url,
                    displayLabel: fd.title,
                });
                showAlert(
                    "success",
                    "Job Listing Created",
                    "New listing was added.",
                );
            } else if (mode === "edit" && existingJobListing) {
                await updateJobListing(existingJobListing.id, {
                    companyId: fd.companyId,
                    title: fd.title,
                    workType: fd.workType,
                    category: fd.category,
                    seniorityLevel: fd.seniorityLevel,
                    salaryMin: fd.salaryMin ?? undefined,
                    salaryMax: fd.salaryMax ?? undefined,
                    currency: fd.currency,
                    description: fd.description,
                    url: fd.url,
                });
                showAlert(
                    "success",
                    "Job Listing Updated",
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
                    ? "Edit Job Listing"
                    : "Create Job Listing"}</Dialog.Title
            >
            <Dialog.Description class="text-sm text-muted-foreground">
                {mode === "edit"
                    ? "Modify job listing details below."
                    : "Provide details to add a new job listing."}
            </Dialog.Description>
        </Dialog.Header>

        <form class="flex flex-col gap-5" onsubmit={handleSubmit}>
            <div>
                <Label>Company</Label><CustomIDSelector
                    sourceStore={companies}
                    bind:selectedId={$formData.companyId}
                />
            </div>

            <div class="grid sm:grid-cols-2 gap-4">
                <div>
                    <Label>Title</Label><Input
                        bind:value={$formData.title}
                        placeholder="Position title"
                    />
                </div>
                <div>
                    <Label>Work Type</Label><CustomEnumSelector
                        enumObject={WorkTypeDisplay}
                        bind:selectedValue={$formData.workType}
                    />
                </div>
            </div>

            <div class="grid sm:grid-cols-2 gap-4">
                <div>
                    <Label>Seniority Level</Label><CustomEnumSelector
                        enumObject={SeniorityLevelDisplay}
                        bind:selectedValue={$formData.seniorityLevel}
                    />
                </div>
                <div>
                    <Label>Currency</Label><CustomEnumSelector
                        enumObject={CurrencyDisplay}
                        bind:selectedValue={$formData.currency}
                    />
                </div>
            </div>

            <div class="grid sm:grid-cols-2 gap-4">
                <div>
                    <Label>Min Salary</Label><Input
                        type="number"
                        bind:value={$formData.salaryMin}
                        placeholder="50000"
                    />
                </div>
                <div>
                    <Label>Max Salary</Label><Input
                        type="number"
                        bind:value={$formData.salaryMax}
                        placeholder="70000"
                    />
                </div>
            </div>

            <div>
                <Label>Description</Label><Textarea
                    bind:value={$formData.description}
                    placeholder="Job description..."
                />
            </div>
            <div>
                <Label>Application URL</Label><Input
                    bind:value={$formData.url}
                    placeholder="https://example.com/job"
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
