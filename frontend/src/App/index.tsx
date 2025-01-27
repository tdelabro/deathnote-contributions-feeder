import { lazy, Suspense } from "react";
import { ErrorBoundary } from "react-error-boundary";
import { useRoutes } from "react-router-dom";

import Layout from "src/components/Layout";
import ProtectedRoute from "src/components/ProtectedRoute";
import ErrorFallback from "src/components/ErrorFallback";

const Login = lazy(() => import("src/pages/Login"));
const Projects = lazy(() => import("src/pages/Projects"));
const Profile = lazy(() => import("src/pages/Profile"));
const MyContributions = lazy(() => import("src/pages/MyContributions"));
const ProjectDetails = lazy(() => import("src/pages/ProjectDetails"));

import { HasuraUserRole } from "src/types";
import LoaderFallback from "src/components/Loader";
import ScrollToTop from "src/components/ScrollToTop";

export enum RoutePaths {
  Projects = "/",
  Login = "/login",
  Profile = "/profile",
  ProjectDetails = "/projects/:projectId",
  MyProjectDetails = "/my-projects/:projectId",
  MyContributions = "/my-contributions",
  CatchAll = "*",
}

function App() {
  const routes = useRoutes([
    {
      element: <Layout />,
      children: [
        {
          path: RoutePaths.Projects,
          element: <Projects />,
        },
        {
          path: RoutePaths.Profile,
          element: (
            <ProtectedRoute requiredRole={HasuraUserRole.RegisteredUser}>
              <Profile />
            </ProtectedRoute>
          ),
        },
        {
          path: RoutePaths.MyContributions,
          element: (
            <ProtectedRoute requiredRole={HasuraUserRole.RegisteredUser}>
              <MyContributions />
            </ProtectedRoute>
          ),
        },
        {
          path: RoutePaths.Login,
          element: <Login />,
        },
        {
          path: RoutePaths.ProjectDetails,
          element: <ProjectDetails />,
        },
        {
          path: RoutePaths.MyProjectDetails,
          element: <ProjectDetails />,
        },
        {
          path: RoutePaths.CatchAll,
          element: <Projects />,
        },
      ],
    },
  ]);

  return (
    <ErrorBoundary FallbackComponent={ErrorFallback}>
      <ScrollToTop />
      <Suspense
        fallback={
          <div className="h-screen">
            <LoaderFallback />
          </div>
        }
      >
        {routes}
      </Suspense>
    </ErrorBoundary>
  );
}

export default App;
