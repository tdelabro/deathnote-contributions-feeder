import Card from "src/components/Card";
import ReactMarkdown from "react-markdown";
import remarkGfm from "remark-gfm";
import { useIntl } from "src/hooks/useIntl";
import { ProjectLeadProps } from "src/components/LeadContributor";
import { Contributor, LanguageMap } from "src/types";
import OverviewPanel from "./OverviewPanel";

interface OverviewProps extends React.PropsWithChildren {
  lead?: ProjectLeadProps | null;
  totalSpentAmountInUsd?: number;
  githubRepoInfo?: {
    decodedReadme?: string;
    owner?: string;
    name?: string;
    contributors?: Contributor[];
    languages: LanguageMap;
  };
}

export default function Overview({ lead, totalSpentAmountInUsd, githubRepoInfo, children }: OverviewProps) {
  const { T } = useIntl();
  return (
    <div className="flex flex-col gap-8 mt-3">
      <div className="text-3xl font-belwe">{T("project.details.overview.title")}</div>
      {children}
      <div className="flex flex-row gap-5">
        {githubRepoInfo?.decodedReadme && (
          <div className="flex-1">
            <Card>
              <div className="font-medium text-lg pb-4">{T("project.details.overview.readmeTitle")}</div>
              <ReactMarkdown skipHtml={true} remarkPlugins={[[remarkGfm]]} className="prose lg:prose-l prose-invert">
                {githubRepoInfo.decodedReadme}
              </ReactMarkdown>
            </Card>
          </div>
        )}
        <Card className="h-fit p-0 basis-96">
          <OverviewPanel {...{ lead, githubRepoInfo, totalSpentAmountInUsd }} />
        </Card>
      </div>
    </div>
  );
}
