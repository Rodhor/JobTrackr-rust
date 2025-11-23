<script lang="ts">
    import { Label } from "$lib/components/ui/label";
    import { Input } from "$lib/components/ui/input";
    import CustomEnumSelector from "$lib/components/formDialogs/utils/CustomEnumSelector.svelte";
    import { WorkType } from "$lib/types/enums";
    import type { Company } from "$lib/types/company";
    import { Button } from "$lib/components/ui/button";
    import { base } from "$app/paths";
    import { goto } from "$app/navigation";
    import { onMount } from "svelte";
    import {
        updateCompany,
        createCompany,
        companies,
    } from "$lib/stores/companies";

    let { companyID }: { companyID?: number } = $props();

    let form = $state<Company>({
        name: "",
        industry: "",
        streetAddress: "",
        zipCode: "",
        country: "",
        defaultWorkType: WorkType.Other,
        website: "",
        phoneNumber: "",
    });

    onMount(() => {
        if (!companyID) return;
        const found = $companies.find((c) => c.id === companyID);
        if (found) Object.assign(form, found);
    });

    const invalidSubmit = $derived(() => !form.name);

    async function submit() {
        if (invalidSubmit()) return; // call it
        if (companyID) {
            await updateCompany(Number(companyID), form);
        } else {
            await createCompany(form);
        }
        console.log(form);
        goto("/companies");
    }
</script>

<section class="w-1/3 mx-auto flex flex-col justify-center gap-3">
    <h1 class="text-xl font-extrabold py-4">Create a new Company</h1>

    <div>
        <Label for="companyName" class="py-2 required">Company Name:</Label>
        <Input
            id="companyName"
            bind:value={form.name}
            placeholder="Enter company name"
        />
    </div>
    <div>
        <Label for="industry" class="py-2">Industry</Label>
        <Input
            id="industry"
            bind:value={form.industry}
            placeholder="Software / Socialwork"
        />
    </div>
    <div>
        <Label for="streetAdress" class="py-2">Street Adress</Label>
        <Input
            id="streetAdress"
            bind:value={form.streetAddress}
            placeholder="Mainstreet 6"
        />
    </div>
    <div>
        <Label for="zipCode" class="py-2">Zip Code</Label>
        <Input id="zipCode" bind:value={form.zipCode} placeholder="12345" />
    </div>
    <div>
        <Label for="country" class="py-2">Country</Label>
        <Input id="country" bind:value={form.country} placeholder="Germany" />
    </div>
    <div>
        <Label for="worktype" class="py-2">Default Worktype</Label>
        <CustomEnumSelector
            enumObject={WorkType}
            bind:selectedValue={form.defaultWorkType as WorkType | undefined}
        />
    </div>
    <div>
        <Label for="website" class="py-2">Website</Label>
        <Input
            id="website"
            bind:value={form.website}
            placeholder="www.google.com"
        />
    </div>
    <div>
        <Label for="phone" class="py-2">Phone Number</Label>
        <Input
            id="phone"
            bind:value={form.phoneNumber}
            placeholder="+49 0123456789"
        />
    </div>
    <div class="flex justify-between mt-4">
        <Button variant="destructive" href="{base}/companies">Cancel</Button>
        <Button disabled={invalidSubmit()} onclick={submit}>Submit</Button>
    </div>
</section>
