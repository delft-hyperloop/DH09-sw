<script lang="ts">
    import { getModalStore } from '@skeletonlabs/skeleton';
    import { modalBody, modalTitle } from '$lib/stores/data';

    let modalStore = getModalStore();

    let closeModal = () => {
        modalStore.close();
    }
</script>

<div class="bg-surface-800 rounded-lg border-surface-600 border-2 max-w-xl">
    <div class="flex flex-col items-center px-16">
        <svg
            viewBox="6 4 36 32"
            fill="none"
            xmlns="http://www.w3.org/2000/svg"
            class="mx-10 my-7 max-w-60"
        >
            <path d="M26.5981 6.5L40.4545 30.5C41.6092 32.5 40.1658 35 37.8564 35H10.1436C7.83419 35 6.39082 32.5 7.54552 30.5L21.4019 6.5C22.5566 4.5 25.4434 4.5 26.5981 6.5Z" stroke="#619A89" stroke-width="2" stroke-linejoin="round"/>
            <path d="M23 16C23 15.4477 23.4477 15 24 15V15C24.5523 15 25 15.4477 25 16V24C25 24.5523 24.5523 25 24 25V25C23.4477 25 23 24.5523 23 24V16Z" fill="#619A89"/>
            <rect x="23" y="27" width="2" height="2" rx="1" fill="#619A89"/>
        </svg>
        <h3 class="text-xl mb-2 font-medium w-full text-center">
            {$modalTitle}
        </h3>
        <span class="text-center text-wrap">
            {$modalBody}
        </span>
        <button
            class="btn w-full rounded-md font-number font-medium text-wrap overflow-auto bg-primary-500 text-surface-900 my-7"
            on:click={closeModal}
        >
            Cancel
        </button>
    </div>
</div>