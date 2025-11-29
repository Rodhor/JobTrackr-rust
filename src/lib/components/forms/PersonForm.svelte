<script lang="ts">
    import { goto } from "$app/navigation";
    import { Button } from "$lib/components/ui/button";
    import { companies } from "$lib/stores/companies";
    import { people, createPerson, updatePerson } from "$lib/stores/people";
    import { Role } from "$lib/types/enums";
    import type { Person } from "$lib/types/person";
    import CustomEnumSelector from "../formDialogs/utils/CustomEnumSelector.svelte";
    import CustomIDSelectCreate from "../formDialogs/utils/CustomIDSelectCreate.svelte";
    import { Input } from "../ui/input";
    import { Label } from "../ui/label";

    let { personID }: { personID?: number } = $props();

    const items = $derived(
        $companies
            .filter((c) => c.id !== undefined)
            .map((c) => ({
                id: c.id!,
                displayLabel: c.name,
            })),
    );

    let form = $state<Person>({
        firstName: "",
        lastName: "",
        email: "",
        phoneNumber: "",
        role: Role.Other,
        linkedinUrl: "",
        companyId: undefined,
    });

    $effect(() => {
        if (!personID) return;
        const found = $people.find((p) => p.id === personID);
        if (found) Object.assign(form, found);
    });

    const invalidSubmit = $derived(
        !form.firstName.trim() || !form.lastName.trim(),
    );

    async function submit() {
        if (invalidSubmit) return;

        if (personID) {
            await updatePerson(Number(personID), form);
        } else {
            await createPerson(form);
        }

        console.log($state.snapshot(form));
        goto("/people");
    }
</script>

<section class="w-1/3 mx-auto flex flex-col justify-center gap-4">
    <h1 class="text-xl font-extrabold">
        {personID ? "Edit Person" : "Create a new Person"}
    </h1>

    <div class="space-y-4">
        <div>
            <Label for="company" class="py-2 required">Company</Label>
            <CustomIDSelectCreate
                {items}
                bind:value={form.companyId}
                caller="people"
                createNew="companies"
            />
        </div>

        <div class="grid grid-cols-2 gap-4">
            <div>
                <Label for="firstName" class="py-2 required">First Name</Label>
                <Input
                    id="firstName"
                    bind:value={form.firstName}
                    placeholder="John"
                />
            </div>
            <div>
                <Label for="lastName" class="py-2 required">Last Name</Label>
                <Input
                    id="lastName"
                    bind:value={form.lastName}
                    placeholder="Doe"
                />
            </div>
        </div>

        <div>
            <Label for="email" class="py-2">Email</Label>
            <Input
                id="email"
                type="email"
                bind:value={form.email}
                placeholder="example@mail.com"
            />
        </div>

        <div>
            <Label for="phoneNumber" class="py-2">Phone</Label>
            <Input
                id="phoneNumber"
                bind:value={form.phoneNumber}
                placeholder="+49 123 456"
            />
        </div>

        <div>
            <Label for="role" class="py-2">Role</Label>
            <CustomEnumSelector
                enumObject={Role}
                bind:selectedValue={form.role}
            />
        </div>

        <div>
            <Label for="linkedinUrl" class="py-2">LinkedIn</Label>
            <Input
                id="linkedinUrl"
                bind:value={form.linkedinUrl}
                placeholder="https://linkedin.com/in/..."
            />
        </div>
    </div>

    <div class="flex justify-between mt-6">
        <Button variant="destructive" href="/people">Cancel</Button>
        <Button disabled={invalidSubmit} onclick={submit}>
            {personID ? "Update" : "Create"}
        </Button>
    </div>
</section>
