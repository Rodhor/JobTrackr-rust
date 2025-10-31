<script lang="ts">
    import * as Dialog from "$lib/components/ui/dialog";
    import { Button } from "$lib/components/ui/button";
    import { Label } from "$lib/components/ui/label";
    import { Input } from "$lib/components/ui/input";
    import CustomAlert from "./utils/CustomAlert.svelte";
    import CustomEnumSelector from "./utils/CustomEnumSelector.svelte";
    import { createCompany, updateCompany } from "$lib/stores/companies";
    import { createDefaultForm } from "./utils/baseForm.svelte";
    import { WorkType } from "$lib/types/enums";
    import type { Company } from "$lib/types/company";

    let {
        open = $bindable(false),
        mode = "create",
        existingCompany = null as Company | null,
    } = $props();

    const defaultForm = {
        name: "",
        streetAddress: "",
        zipCode: "",
        city: "",
        country: "",
        defaultWorkType: WorkType.InOffice,
        industry: "",
        website: "",
        phoneNumber: "",
    };
    const { formData, resetForm } = createDefaultForm(defaultForm);

    $effect(() => {
        if (mode === "edit" && existingCompany) {
            $formData = {
                name: existingCompany.name ?? "",
                streetAddress: existingCompany.streetAddress ?? "",
                zipCode: existingCompany.zipCode ?? "",
                city: existingCompany.city ?? "",
                country: existingCompany.country ?? "",
                defaultWorkType:
                    existingCompany.defaultWorkType ?? WorkType.InOffice,
                industry: existingCompany.industry ?? "",
                website: existingCompany.website ?? "",
                phoneNumber: existingCompany.phoneNumber ?? "",
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
                await createCompany({
                    name: fd.name,
                    streetAddress: fd.streetAddress,
                    zipCode: fd.zipCode,
                    city: fd.city,
                    country: fd.country,
                    defaultWorkType: fd.defaultWorkType,
                    industry: fd.industry,
                    website: fd.website,
                    phoneNumber: fd.phoneNumber,
                    displayLabel: fd.name.trim(),
                });
                showAlert("success", "Company Created", "New company added.");
            } else if (mode === "edit" && existingCompany) {
                await updateCompany(existingCompany.id, {
                    name: fd.name,
                    streetAddress: fd.streetAddress,
                    zipCode: fd.zipCode,
                    city: fd.city,
                    country: fd.country,
                    defaultWorkType: fd.defaultWorkType,
                    industry: fd.industry,
                    website: fd.website,
                    phoneNumber: fd.phoneNumber,
                });
                showAlert(
                    "success",
                    "Company Updated",
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
                    ? "Edit Company"
                    : "Create Company"}</Dialog.Title
            >
            <Dialog.Description class="text-sm text-muted-foreground">
                {mode === "edit"
                    ? "Modify company details below."
                    : "Provide details to add a new company."}
            </Dialog.Description>
        </Dialog.Header>

        <form class="flex flex-col gap-5" onsubmit={handleSubmit}>
            <div class="grid sm:grid-cols-2 gap-4">
                <div>
                    <Label>Name</Label><Input
                        bind:value={$formData.name}
                        placeholder="Company GmbH"
                        required
                    />
                </div>
                <div>
                    <Label>Industry</Label><Input
                        bind:value={$formData.industry}
                        placeholder="Software / Consulting"
                    />
                </div>
            </div>

            <div class="grid sm:grid-cols-2 gap-4">
                <div>
                    <Label>Street</Label><Input
                        bind:value={$formData.streetAddress}
                        placeholder="Main Street 1"
                    />
                </div>
                <div>
                    <Label>ZIP</Label><Input
                        bind:value={$formData.zipCode}
                        placeholder="12345"
                    />
                </div>
                <div>
                    <Label>City</Label><Input
                        bind:value={$formData.city}
                        placeholder="Berlin"
                    />
                </div>
                <div>
                    <Label>Country</Label><Input
                        bind:value={$formData.country}
                        placeholder="Germany"
                    />
                </div>
            </div>

            <div>
                <Label>Default Work Type</Label><CustomEnumSelector
                    enumObject={WorkType}
                    bind:selectedValue={$formData.defaultWorkType}
                />
            </div>

            <div class="grid sm:grid-cols-2 gap-4">
                <div>
                    <Label>Website</Label><Input
                        bind:value={$formData.website}
                        placeholder="https://example.com"
                    />
                </div>
                <div>
                    <Label>Phone</Label><Input
                        bind:value={$formData.phoneNumber}
                        placeholder="+49 0123456789"
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
