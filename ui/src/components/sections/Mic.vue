<template>
  <ContentContainer>
    <CenteredContainer>
      <Gate ref="gate" />
      <Equaliser/>
      <Compressor/>
      <MicExtra />
    </CenteredContainer>
  </ContentContainer>
</template>

<script>
import Gate from "./mic/Gate.vue";
import Equaliser from "./mic/Equalizer.vue";
import Compressor from "./mic/Compressor.vue";
import MicExtra from "./mic/MicExtra.vue";
import ContentContainer from "@/components/containers/ContentContainer.vue";
import CenteredContainer from "@/components/containers/CenteredContainer.vue";

export default {
  name: "MicSections",
  components: {
    ContentContainer,
    CenteredContainer,
    MicExtra, Compressor, Equaliser, Gate},
}
</script>

<style scoped>
</style>
