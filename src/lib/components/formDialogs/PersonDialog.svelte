<script lang="ts">
    import * as Dialog from "$lib/components/ui/dialog";
    import { Button } from "$lib/components/ui/button";
    import { Label } from "$lib/components/ui/label";
    import { Input } from "$lib/components/ui/input";
    import CustomEnumSelector from "./utils/CustomEnumSelector.svelte";
    import CustomAlert from "./utils/CustomAlert.svelte";
    import CustomIDSelector from "./utils/CustomIDSelector.svelte";
    import { createPerson, updatePerson } from "$lib/stores/people";
    import { companies } from "$lib/stores/companies";
    import { createDefaultForm } from "./utils/baseForm.svelte";
    import { Role } from "$lib/types/enums";
    import type { Person } from "$lib/types/person";

    let {
        open = $bindable(false),
        mode = "create",
        existingPerson = null as Person | null,
    } = $props();

    const defaultForm = {
        firstName: "",
        lastName: "",
        email: "",
        phoneNumber: "",
        role: Role.Other,
        linkedinUrl: "",
        companyId: null as number | null,
    };
    const { formData, resetForm } = createDefaultForm(defaultForm);

    $effect(() => {
        if (mode === "edit" && existingPerson) {
            $formData = {
                firstName: existingPerson.firstName ?? "",
                lastName: existingPerson.lastName ?? "",
                email: existingPerson.email ?? "",
                phoneNumber: existingPerson.phoneNumber ?? "",
                role: existingPerson.role ?? Role.Other,
                linkedinUrl: existingPerson.linkedinUrl ?? "",
                companyId: existingPerson.companyId ?? null,
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
        try {
            if (mode === "create") {
                await createPerson({
                    firstName: fd.firstName,
                    lastName: fd.lastName,
                    email: fd.email || undefined,
                    phoneNumber: fd.phoneNumber || undefined,
                    role: fd.role,
                    linkedinUrl: fd.linkedinUrl || undefined,
                    companyId: fd.companyId ?? undefined,
                    displayLabel: `${fd.firstName} ${fd.lastName}`.trim(),
                });
                showAlert(
                    "success",
                    "Person Created",
                    "A new contact was added.",
                );
            } else if (mode === "edit" && existingPerson) {
                await updatePerson(existingPerson.id, {
                    firstName: fd.firstName,
                    lastName: fd.lastName,
                    email: fd.email || undefined,
                    phoneNumber: fd.phoneNumber || undefined,
                    role: fd.role,
                    linkedinUrl: fd.linkedinUrl || undefined,
                    companyId: fd.companyId ?? undefined,
                    displayLabel: `${fd.firstName} ${fd.lastName}`.trim(),
                });
                showAlert(
                    "success",
                    "Person Updated",
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
                    ? "Edit Person"
                    : "Create Person"}</Dialog.Title
            >
            <Dialog.Description class="text-sm text-muted-foreground">
                {mode === "edit"
                    ? "Modify person details below."
                    : "Provide details to add a new contact."}
            </Dialog.Description>
        </Dialog.Header>

        <form class="flex flex-col gap-5" onsubmit={handleSubmit}>
            <div class="grid sm:grid-cols-2 gap-4">
                <div>
                    <Label>First Name</Label><Input
                        bind:value={$formData.firstName}
                        placeholder="John"
                    />
                </div>
                <div>
                    <Label>Last Name</Label><Input
                        bind:value={$formData.lastName}
                        placeholder="Doe"
                    />
                </div>
            </div>

            <div class="grid sm:grid-cols-2 gap-4">
                <div>
                    <Label>Email</Label><Input
                        type="email"
                        bind:value={$formData.email}
                        placeholder="example@mail.com"
                    />
                </div>
                <div>
                    <Label>Phone</Label><Input
                        bind:value={$formData.phoneNumber}
                        placeholder="+49 123 456"
                    />
                </div>
            </div>

            <div class="grid sm:grid-cols-2 gap-4">
                <div>
                    <Label>Role</Label><CustomEnumSelector
                        enumObject={Role}
                        bind:selectedValue={$formData.role}
                    />
                </div>
                <div>
                    <Label>LinkedIn</Label><Input
                        bind:value={$formData.linkedinUrl}
                        placeholder="https://linkedin.com/in/..."
                    />
                </div>
            </div>

            <div>
                <Label>Company</Label><CustomIDSelector
                    sourceStore={companies}
                    bind:selectedId={$formData.companyId}
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
