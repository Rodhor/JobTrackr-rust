<script lang="ts">
    import * as Popover from "$lib/components/ui/popover";
    import { cn } from "$lib/utils";
    import {
        type DateValue,
        getLocalTimeZone,
        parseDate,
    } from "@internationalized/date";
    import CalendarIcon from "@lucide/svelte/icons/calendar";
    import { Calendar } from "$lib/components/ui/calendar/index.js";
    import { buttonVariants } from "$lib/components/ui/button";

    let {
        selectedDate = $bindable<string | undefined>(undefined),
    }: {
        selectedDate: string | undefined;
    } = $props();

    let internalDate = $state<DateValue | undefined>(
        selectedDate ? parseDate(selectedDate) : undefined,
    );

    $effect(() => {
        if (!internalDate) {
            selectedDate = undefined;
            return;
        }

        const d = internalDate.toDate(getLocalTimeZone());

        const yyyy = d.getFullYear();
        const mm = String(d.getMonth() + 1).padStart(2, "0");
        const dd = String(d.getDate()).padStart(2, "0");

        selectedDate = `${yyyy}-${mm}-${dd}`;
    });

    let contentRef = $state<HTMLElement | null>(null);
</script>

<Popover.Root>
    <Popover.Trigger
        class={cn(
            buttonVariants({ variant: "outline" }),
            "!flex items-center justify-between gap-2",
            !internalDate && "text-muted-foreground",
        )}
    >
        <CalendarIcon class="size-4 opacity-60" />
        {selectedDate ?? "Pick a date"}
    </Popover.Trigger>

    <Popover.Content bind:ref={contentRef} class="w-auto p-0">
        <Calendar type="single" bind:value={internalDate} />
    </Popover.Content>
</Popover.Root>
