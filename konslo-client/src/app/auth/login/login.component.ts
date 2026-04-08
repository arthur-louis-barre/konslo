import {Component, inject} from '@angular/core';
import {Router, RouterLink} from "@angular/router";
import {AuthService} from "../auth.service";
import {FormBuilder, FormsModule, ReactiveFormsModule, Validators} from "@angular/forms";
import {takeUntilDestroyed} from "@angular/core/rxjs-interop";
import {merge} from "rxjs";

@Component({
    selector: 'k-login',
    imports: [
        FormsModule,
        ReactiveFormsModule,
        RouterLink
    ],
    templateUrl: './login.component.html',
    styleUrl: './login.component.css',
})
export class LoginComponent {
    private router = inject(Router);
    private authService = inject(AuthService);
    private fb = inject(FormBuilder);

    protected isLoading = false;
    protected areCredentialsWrong = false;

    form = this.fb.group({
        email: ['', Validators.required],
        password: ['', Validators.required]
    });

    constructor() {
        merge(
            this.form.controls.email.valueChanges,
            this.form.controls.password.valueChanges
        )
            .pipe(takeUntilDestroyed())
            .subscribe(() => {
                this.areCredentialsWrong = false;
            });
    }

    onSubmit() {
        if (this.isLoading) return;
        this.isLoading = true;

        this.authService
            .login(this.form.value.email!, this.form.value.password!)
            .subscribe({
                next: () => {
                    this.isLoading = false;
                    this.router.navigate(['/habits']);
                },
                error: (err) => {
                    console.error(err);
                    this.isLoading = false
                    this.areCredentialsWrong = true;
                }
            })
    }


}
