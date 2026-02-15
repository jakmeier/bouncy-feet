<script>
  import LoginRequiredContent from '$lib/components/profile/LoginRequiredContent.svelte';
  import StandardPage from '$lib/components/ui/StandardPage.svelte';
  import CreateCombo from '$lib/components/combo/CreateCombo.svelte';
  import { t } from '$lib/i18n';
</script>

<LoginRequiredContent reason={$t('profile.upload.requires-login-description')}>
  {#snippet children({ fullUser, apiUser })}
    <StandardPage mainColor backButton title="profile.combo.create-title">
      <CreateCombo {fullUser} {apiUser} />
    </StandardPage>
  {/snippet}
</LoginRequiredContent>
