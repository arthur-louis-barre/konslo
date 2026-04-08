import {AuthService} from "./auth.service";
import {Router} from "@angular/router";
import {inject} from "@angular/core";
import {catchError, map, of} from "rxjs";

export const authGuard = () => {
    const authService = inject(AuthService);
    const router = inject(Router);

    return authService.me().pipe(
        map(() => true),
        catchError(() => of(router.createUrlTree(['/login'])))
    );
}
