import { ComponentStory, ComponentMeta } from "@storybook/react";
import { ProjectDetailsTab } from "../ProjectDetailsContext";
import View from "./View";

export default {
  title: "ProjectsSidebar",
  component: View,
} as ComponentMeta<typeof View>;

const availableTabs = [ProjectDetailsTab.Overview, ProjectDetailsTab.Contributors, ProjectDetailsTab.Payments];
const currentProject = {
  id: "test-projct-id",
  name: "Our project",
  logoUrl: "https://avatars.githubusercontent.com/u/98735558?v=4",
  leads: [
    {
      id: "leader-id",
      displayName: "Leader",
      avatarUrl: "https://avatars.githubusercontent.com/u/98735558?v=4",
    },
  ],
  nbContributors: 4,
  withInvitation: false,
};
// eslint-disable-next-line @typescript-eslint/no-empty-function
const empty = () => {};
const selectedTab = ProjectDetailsTab.Overview;
const expandable = true;
const allProjects = [currentProject, currentProject, currentProject];

const Template: ComponentStory<typeof View> = () => (
  <View
    {...{
      expandable,
      currentProject,
      allProjects,
      onProjectSelected: empty,
      selectedTab,
      availableTabs,
      dispatch: empty,
    }}
  />
);

export const Default = Template.bind({});
