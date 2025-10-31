<script lang="ts">
    import * as Alert from "$lib/components/ui/alert/index.js";
    import CheckCircle2Icon from "@lucide/svelte/icons/check-circle-2";
    import AlertCircleIcon from "@lucide/svelte/icons/alert-circle";
    import PopcornIcon from "@lucide/svelte/icons/popcorn";

    const { status, message, description } = $props<{
        status: "success" | "error" | "info";
        message: string;
        description?: string;
    }>();
</script>

<div class="grid w-full max-w-xl items-start gap-4">
    {#if status === "error"}
        <Alert.Root variant="destructive">
            <AlertCircleIcon />
            <Alert.Title>{message}</Alert.Title>
            <Alert.Description>{description}</Alert.Description>
        </Alert.Root>
    {:else if status === "success"}
        <Alert.Root>
            <CheckCircle2Icon />
            <Alert.Title>{message}</Alert.Title>
        </Alert.Root>
    {:else}
        <Alert.Root>
            <PopcornIcon />
            <Alert.Title>{message}</Alert.Title>
        </Alert.Root>
    {/if}
</div>
