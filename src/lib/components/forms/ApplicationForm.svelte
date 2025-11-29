<script lang="ts">
    import { Label } from "$lib/components/ui/label";
    import CustomEnumSelector from "$lib/components/formDialogs/utils/CustomEnumSelector.svelte";
    import { Stage } from "$lib/types/enums";
    import { Button } from "$lib/components/ui/button";
    import { goto } from "$app/navigation";
    import {
        createApplication,
        updateApplication,
        applications,
    } from "$lib/stores/applications";
    import type { Application } from "$lib/types/application";
    import CustomIDSelector from "../formDialogs/utils/CustomIDSelector.svelte";
    import { jobListings } from "$lib/stores/jobListings";
    import CustomDatePicker from "../formDialogs/utils/CustomDatePicker.svelte";
    import { Textarea } from "../ui/textarea";
    import { newlyCreatedApplicationId } from "$lib/stores/formState";

    let {
        applicationID,
        caller,
    }: { applicationID?: number; caller?: string | null } = $props();

    let form = $state<Application>({
        jobListingId: undefined,
        stage: Stage.Applied,
        appliedDate: "",
        applicationNotes: "",
    });

    $effect(() => {
        if (!applicationID) return;
        const found = $applications.find((a) => a.id === applicationID);
        if (found) Object.assign(form, found);
    });

    const jobListingItems = $derived(() =>
        $jobListings
            .filter((j) => j.id !== undefined)
            .map((j) => ({
                id: j.id!,
                displayLabel: j.title,
            })),
    );
    const invalidSubmit = $derived(() => !form.appliedDate);

    async function submit() {
        if (invalidSubmit()) return;

        let createdApplication: Application;

        if (applicationID) {
            await updateApplication(Number(applicationID), form);
            createdApplication = form;
        } else {
            createdApplication = await createApplication(form);
        }
        console.log($state.snapshot(form));
        if (caller && !applicationID) {
            newlyCreatedApplicationId.set(createdApplication.id);
        }

        if (caller) {
            goto(`/${caller}/create`);
        } else {
            goto("/applications");
        }
    }
</script>

<section class="w-1/3 mx-auto flex flex-col justify-center gap-3">
    <h1 class="text-xl font-extrabold py-4">Create a new Application</h1>

    <div>
        <Label for="joblisting" class="py-2">Joblisting</Label>
        <CustomIDSelector
            items={jobListingItems()}
            bind:selectedId={form.jobListingId}
        />
    </div>

    <div>
        <Label for="stage" class="py-2">Stage</Label>
        <CustomEnumSelector
            enumObject={Stage}
            bind:selectedValue={form.stage as Stage | undefined}
        />
    </div>
    <div>
        <Label for="applicationDate" class="py-2 required">Date Applied</Label
        ><CustomDatePicker bind:selectedDate={form.appliedDate} />
    </div>

    <div>
        <Label for="notes" class="py-2">Notes</Label>
        <Textarea
            class="!min-h-30"
            bind:value={form.applicationNotes}
            placeholder="Add notes or context..."
        />
    </div>
    <div class="flex justify-between mt-4">
        <Button
            variant="destructive"
            href={caller ? `/${caller}/create?` : "/applications"}
        >
            Cancel
        </Button>
        <Button disabled={invalidSubmit()} onclick={submit}>Submit</Button>
    </div>
</section>
