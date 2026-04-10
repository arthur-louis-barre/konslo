import {Component, inject} from '@angular/core';
import {Router, RouterLink} from "@angular/router";
import {AuthService} from "../auth.service";
import {AbstractControl, FormBuilder, FormsModule, ReactiveFormsModule, ValidationErrors, Validators} from "@angular/forms";
import {takeUntilDestroyed} from "@angular/core/rxjs-interop";
import {merge} from "rxjs";

function passwordsMatch(group: AbstractControl): ValidationErrors | null {
    const password = group.get('password')?.value;
    const confirm = group.get('confirmPassword')?.value;
    return password === confirm ? null : { passwordMismatch: true };
}

@Component({
    selector: 'k-register',
    imports: [FormsModule, ReactiveFormsModule, RouterLink],
    templateUrl: './register.component.html',
    styleUrl: './register.component.css',
})
export class RegisterComponent {
    private router = inject(Router);
    private authService = inject(AuthService);
    private fb = inject(FormBuilder);

    protected isLoading = false;
    protected isPasswordMismatch = false;
    protected isConflict = false;

    form = this.fb.group({
        username: ['', Validators.required],
        password: ['', Validators.required],
        confirmPassword: ['', Validators.required]
    }, { validators: passwordsMatch });

    constructor() {
        this.form.controls.username.valueChanges
            .pipe(takeUntilDestroyed())
            .subscribe(() => this.isConflict = false);

        merge(
            this.form.controls.password.valueChanges,
            this.form.controls.confirmPassword.valueChanges
        )
            .pipe(takeUntilDestroyed())
            .subscribe(() => this.isPasswordMismatch = false);
    }

    onSubmit() {
        if (this.isLoading) return;

        if (this.form.errors?.['passwordMismatch']) {
            this.isPasswordMismatch = true;
            return;
        }

        if (this.form.invalid) return;

        this.isLoading = true;

        this.authService
            .register(this.form.value.username!, this.form.value.password!)
            .subscribe({
                next: () => {
                    this.isLoading = false;
                    this.router.navigate(['/habits']);
                },
                error: (err) => {
                    this.isLoading = false;
                    if (err.status === 409) {
                        this.isConflict = true;
                    }
                }
            });
    }
}
