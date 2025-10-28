<script lang="ts">
    import * as Popover from "$lib/components/ui/popover";
    import { cn } from "$lib/utils";
    import {
        DateFormatter,
        type DateValue,
        getLocalTimeZone,
    } from "@internationalized/date";
    import CalendarIcon from "@lucide/svelte/icons/calendar";
    import { Calendar } from "$lib/components/ui/calendar/index.js";
    import { buttonVariants } from "$lib/components/ui/button";

    // props
    let {
        selectedDate = $bindable<DateValue | undefined>(undefined),
        formatLocale = "de-DE",
    }: {
        selectedDate: DateValue | undefined;
        formatLocale?: string;
    } = $props();

    // internal
    const df = new DateFormatter(formatLocale, { dateStyle: "long" });
    let contentRef = $state<HTMLElement | null>(null);

    // derived ISO value for backend or form use
    const isoDate = $derived(
        selectedDate
            ? selectedDate.toDate(getLocalTimeZone()).toISOString()
            : "",
    );
</script>

<Popover.Root>
    <Popover.Trigger
        class={cn(
            buttonVariants({ variant: "outline" }),
            "!flex items-center justify-between gap-2",
            !selectedDate && "text-muted-foreground",
        )}
    >
        <CalendarIcon class="size-4 opacity-60" />
        {selectedDate
            ? df.format(selectedDate.toDate(getLocalTimeZone()))
            : "Pick a date"}
    </Popover.Trigger>

    <Popover.Content bind:ref={contentRef} class="w-auto p-0">
        <Calendar type="single" bind:value={selectedDate} />
    </Popover.Content>
</Popover.Root>
