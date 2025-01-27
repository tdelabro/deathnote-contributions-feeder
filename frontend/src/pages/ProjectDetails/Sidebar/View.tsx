import { Listbox } from "@headlessui/react";
import Sidebar from "src/components/Sidebar";
import UpDownChevrons from "src/assets/icons/UpDownChevrons";
import GithubLink from "src/components/GithubLink";
import TelegramLink from "src/components/TelegramLink";
import ShootingStar from "src/assets/icons/ShootingStar";
import { RoutePaths } from "src/App";
import BackLink from "src/components/BackLink";
import CheckLine from "src/icons/CheckLine";
import RoundedImage, { ImageSize } from "src/components/RoundedImage";
import { ProjectDetails } from "..";
import { useIntl } from "src/hooks/useIntl";
import { buildGithubLink } from "src/utils/stringUtils";
import {
  PaymentAction,
  ProjectDetailsAction,
  ProjectDetailsActionType,
  ProjectDetailsTab,
} from "../ProjectDetailsContext";

interface Props {
  expandable: boolean;
  currentProject: ProjectDetails;
  allProjects: SidebarProjectDetails[];
  onProjectSelected: (projectId: string) => void;
  availableTabs: ProjectDetailsTab[];
  selectedTab: ProjectDetailsTab;
  dispatch: (action: ProjectDetailsAction) => void;
}

interface SidebarProjectDetails {
  id: string;
  name: string;
  logoUrl: string;
  nbContributors: number;
  withInvitation: boolean;
}

export default function View({
  expandable,
  currentProject,
  allProjects,
  onProjectSelected,
  availableTabs,
  selectedTab,
  dispatch,
}: Props) {
  const { T } = useIntl();

  return (
    <Sidebar>
      {!expandable && (
        <BackLink to={RoutePaths.Projects} className="divide-none">
          {T("project.details.sidebar.backToProjects")}
        </BackLink>
      )}
      <div className="flex flex-col gap-6 divide-y divide-neutral-700 w-full">
        <div className="relative h-16">
          <Listbox
            value={currentProject}
            onChange={project => {
              onProjectSelected(project.id);
            }}
            disabled={!expandable}
          >
            <div className="flex flex-col w-full border-2 rounded-2xl border-neutral-700 divide-y divide-neutral-700 bg-white/2 absolute backdrop-blur-4xl z-10">
              <Listbox.Button className={`p-4 font-medium text-2xl ${expandable ? "hover:cursor-pointer" : ""}`}>
                <div className="flex flex-row gap-3 items-center">
                  <RoundedImage src={currentProject.logoUrl} alt="Project Logo" size={ImageSize.Medium} />
                  <div className="truncate grow font-belwe text-left">{currentProject.name}</div>
                  {expandable && <UpDownChevrons className="h-5 w-5 fill-gray-400" />}
                </div>
              </Listbox.Button>
              <Listbox.Options className="flex flex-col divide-y">
                {allProjects.map(project => (
                  <Listbox.Option
                    key={project.id}
                    value={project}
                    className={`hover:cursor-pointer p-4 hover:bg-white/10 border-neutral-600 duration-300 last:rounded-b-2xl ${
                      project.withInvitation && "bg-orange-400/10  hover:bg-amber-700/30"
                    } `}
                  >
                    <div className="flex flex-col gap-5">
                      <div className="flex flex-row gap-5 items-center">
                        <RoundedImage src={project.logoUrl} alt="Project Logo" />
                        <div className="flex flex-col flex-1 justify-self-start truncate">
                          <div className="truncate text-base font-medium">{project.name}</div>
                          <div className="truncate text-sm font-regular text-slate-400">
                            {project.nbContributors} {T("project.details.sidebar.contributors")}
                          </div>
                        </div>
                        <>
                          {project.withInvitation ? (
                            <div className="flex flex-row px-2 py-1 rounded-2xl bg-orange-400 items-center gap-1 text-xs text-black">
                              <ShootingStar />
                              <div>{T("project.details.sidebar.newInvite")}</div>
                            </div>
                          ) : (
                            project.id === currentProject.id && (
                              <CheckLine className="text-gray-200 text-lg font-normal" />
                            )
                          )}
                        </>
                      </div>
                      {project.withInvitation && (
                        <div className="bg-neutral-100 rounded-xl w-full text-black text-sm text-center p-2">
                          View invite
                        </div>
                      )}
                    </div>
                  </Listbox.Option>
                ))}
              </Listbox.Options>
            </div>
          </Listbox>
        </div>
        <div className="flex flex-col align-start font-medium text-xl pt-3 pb-2 gap-2">
          {availableTabs.map(tab => (
            <div
              key={tab}
              className={`rounded-xl hover:cursor-pointer text-white text-base px-4 py-2.5 ${
                selectedTab === tab ? "bg-white/8" : "text-neutral-400"
              }`}
              onClick={() =>
                dispatch(
                  tab !== ProjectDetailsTab.Payments
                    ? { type: ProjectDetailsActionType.SelectTab, selectedTab: tab }
                    : { type: ProjectDetailsActionType.SelectPaymentAction, selectedPaymentAction: PaymentAction.List }
                )
              }
              data-testid={`${tab}-tab`}
            >
              {tab}
            </div>
          ))}
        </div>
        <div className="flex flex-row gap-2 pt-8">
          {currentProject.telegramLink && <TelegramLink link={currentProject.telegramLink} />}
          {currentProject.githubRepoInfo?.owner && currentProject.githubRepoInfo.name && (
            <GithubLink
              link={buildGithubLink(currentProject.githubRepoInfo.owner, currentProject.githubRepoInfo.name)}
            />
          )}
        </div>
      </div>
    </Sidebar>
  );
}
