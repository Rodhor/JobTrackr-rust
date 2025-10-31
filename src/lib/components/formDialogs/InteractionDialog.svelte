<script lang="ts">
    import * as Dialog from "$lib/components/ui/dialog";
    import { Button } from "$lib/components/ui/button";
    import { Label } from "$lib/components/ui/label";
    import { Input } from "$lib/components/ui/input";
    import { Textarea } from "$lib/components/ui/textarea";
    import CustomDatePicker from "./utils/CustomDatePicker.svelte";
    import CustomEnumSelector from "./utils/CustomEnumSelector.svelte";
    import CustomAlert from "./utils/CustomAlert.svelte";
    import CustomIDSelector from "./utils/CustomIDSelector.svelte";
    import { applications } from "$lib/stores/applications";
    import { people } from "$lib/stores/people";
    import { companies } from "$lib/stores/companies";
    import {
        createInteraction,
        updateInteraction,
    } from "$lib/stores/interactions";
    import { createDefaultForm } from "./utils/baseForm.svelte";
    import { InteractionType } from "$lib/types/enums";
    import type { Interaction } from "$lib/types/interaction";
    import { parseDate, getLocalTimeZone } from "@internationalized/date";

    let {
        open = $bindable(false),
        mode = "create",
        existingInteraction = null as Interaction | null,
    } = $props();

    const defaultForm = {
        interactionType: InteractionType.Email,
        interactionDate: parseDate(new Date().toISOString().split("T")[0]),
        subject: "",
        summary: "",
        medium: "",
        applicationId: null as number | null,
        personId: null as number | null,
        companyId: null as number | null,
    };
    const { formData, resetForm } = createDefaultForm(defaultForm);

    $effect(() => {
        if (mode === "edit" && existingInteraction) {
            $formData = {
                interactionType:
                    existingInteraction.interactionType ??
                    InteractionType.Email,
                interactionDate: parseDate(existingInteraction.interactionDate),
                subject: existingInteraction.subject ?? "",
                summary: existingInteraction.summary ?? "",
                medium: existingInteraction.medium ?? "",
                applicationId: existingInteraction.applicationId ?? null,
                personId: existingInteraction.personId ?? null,
                companyId: existingInteraction.companyId ?? null,
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
        const interactionDate = fd.interactionDate
            ? `${fd.interactionDate.year}-${String(fd.interactionDate.month).padStart(2, "0")}-${String(fd.interactionDate.day).padStart(2, "0")}`
            : new Date().toISOString().split("T")[0];

        try {
            if (mode === "create") {
                await createInteraction({
                    interactionType: fd.interactionType,
                    interactionDate,
                    subject: fd.subject,
                    summary: fd.summary,
                    medium: fd.medium,
                    applicationId: fd.applicationId ?? undefined,
                    personId: fd.personId ?? undefined,
                    companyId: fd.companyId ?? undefined,
                    displayLabel: `Interaction: ${fd.subject || fd.interactionType}`,
                });
                showAlert(
                    "success",
                    "Interaction Created",
                    "New interaction logged.",
                );
            } else if (mode === "edit" && existingInteraction) {
                await updateInteraction(existingInteraction.id, {
                    interactionType: fd.interactionType,
                    interactionDate,
                    subject: fd.subject,
                    summary: fd.summary,
                    medium: fd.medium,
                    applicationId: fd.applicationId ?? undefined,
                    personId: fd.personId ?? undefined,
                    companyId: fd.companyId ?? undefined,
                });
                showAlert(
                    "success",
                    "Interaction Updated",
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
                    ? "Edit Interaction"
                    : "Create Interaction"}</Dialog.Title
            >
            <Dialog.Description class="text-sm text-muted-foreground">
                {mode === "edit"
                    ? "Modify interaction details below."
                    : "Provide details to log a new interaction."}
            </Dialog.Description>
        </Dialog.Header>

        <form class="flex flex-col gap-5" onsubmit={handleSubmit}>
            <div class="grid sm:grid-cols-2 gap-4">
                <div>
                    <Label>Type</Label><CustomEnumSelector
                        enumObject={InteractionType}
                        bind:selectedValue={$formData.interactionType}
                    />
                </div>
                <div>
                    <Label>Date</Label><CustomDatePicker
                        bind:selectedDate={$formData.interactionDate}
                    />
                </div>
            </div>

            <div>
                <Label>Subject</Label><Input
                    bind:value={$formData.subject}
                    placeholder="Call with HR"
                />
            </div>
            <div>
                <Label>Summary</Label><Textarea
                    bind:value={$formData.summary}
                    placeholder="Details about the interaction..."
                />
            </div>
            <div>
                <Label>Medium</Label><Input
                    bind:value={$formData.medium}
                    placeholder="Zoom / Email / Office"
                />
            </div>

            <div class="grid sm:grid-cols-3 gap-4">
                <div>
                    <Label>Application</Label><CustomIDSelector
                        sourceStore={applications}
                        bind:selectedId={$formData.applicationId}
                    />
                </div>
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
