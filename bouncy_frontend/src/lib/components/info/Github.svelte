<script>
  import { t } from '$lib/i18n';
</script>

<div class="space">
  <div class="transparent-box">
    <div>
      {$t('home.go-to-github')}
    </div>
    <div class="centered small-space">
      <a href="https://github.com/jakmeier/bouncy-feet/issues">
        <button> GitHub </button>
      </a>
    </div>
  </div>
</div>

<style>
  .transparent-box {
    border-radius: 10px;
    margin: 12rem 0;
    z-index: 1;
    position: relative;
    color: var(--theme-neutral-dark);
  }

  .centered {
    text-align: center;
  }

  button {
    margin: 5px;
    width: fit-content;
    height: fit-content;
  }

  .small-space {
    margin: 1rem 0px;
  }
  .space {
    margin: 3rem 0px;
  }
</style>
