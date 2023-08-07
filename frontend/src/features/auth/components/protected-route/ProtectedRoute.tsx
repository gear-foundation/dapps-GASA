import { Navigate, useLocation } from 'react-router';
import { ROUTES } from 'consts';
import { useAccount } from '@gear-js/react-hooks';
import { ProtectedRouteProps } from './ProtectedRoute.interface';
import { useAuth } from '../../hooks';

function ProtectedRoute({ children }: ProtectedRouteProps) {
  const { authToken } = useAuth();
  const { account } = useAccount();
  const location = useLocation();

  if (!authToken && account) {
    return <Navigate to={`/${ROUTES.NOT_AUTHORIZED}`} replace />;
  }

  if (!authToken && !account) {
    return <Navigate to={`/${ROUTES.LOGIN}`} state={{ from: location }} replace />;
  }

  return children;
}

export { ProtectedRoute };
