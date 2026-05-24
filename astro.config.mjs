import { defineConfig } from "astro/config";
import starlight from "@astrojs/starlight";

export default defineConfig({
  site: "https://docs.noeracle.org",
  integrations: [
    starlight({
      title: "Noeracle",
      description: "On-demand price oracle for Stellar.",
      favicon: "/favicon.svg",
      social: {
        github: "https://github.com/noeracle/noeracle",
      },
      customCss: ["./src/styles/custom.css"],
      editLink: {
        baseUrl: "https://github.com/noeracle/docs/edit/main/",
      },
      lastUpdated: true,
      sidebar: [
        { label: "Introduction", link: "/" },
        { label: "Quickstart", link: "/quickstart/" },
        { label: "Integration", link: "/integration/" },
        {
          label: "Reference",
          items: [
            { label: "SDK", link: "/reference/sdk/" },
            { label: "Contract", link: "/reference/contract/" },
          ],
        },
        {
          label: "Design",
          items: [
            { label: "Architecture", link: "/architecture/" },
            { label: "Threat model", link: "/threat-model/" },
            { label: "Roadmap", link: "/roadmap/" },
          ],
        },
      ],
    }),
  ],
});
