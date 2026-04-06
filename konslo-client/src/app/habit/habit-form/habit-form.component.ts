import {Component, ElementRef, inject, viewChild} from '@angular/core';
import {FormControl, FormGroup, ReactiveFormsModule, Validators} from '@angular/forms';
import {HabitService} from '../habit.service';
import {GoalPeriod} from "../habit.model";
import {Router} from "@angular/router";
import {takeUntilDestroyed} from "@angular/core/rxjs-interop";

@Component({
    selector: 'k-new-habit-form',
    imports: [ReactiveFormsModule],
    templateUrl: './habit-form.component.html',
    styleUrl: './habit-form.component.css'
})
export class HabitFormComponent {
    private router = inject(Router);
    private habitsService = inject(HabitService);

    protected isLoading = false;
    protected nameConflict = false;

    private nameInput = viewChild.required<ElementRef>('nameInput');

    form = new FormGroup({
        name: new FormControl('', Validators.required),
        goal_value: new FormControl(1, [Validators.required, Validators.min(1)]),
        goal_unit: new FormControl('', Validators.required),
        goal_period: new FormControl('day', Validators.required),
    });

    constructor() {
        this.form.controls.name.valueChanges
            .pipe(takeUntilDestroyed())
            .subscribe(() => {
                this.nameConflict = false;
            });
    }

    onSubmit() {
        if (this.isLoading) return;
        this.isLoading = true;

        this.habitsService.createHabit({
            name: this.form.value.name!,
            goal_value: this.form.value.goal_value!,
            goal_unit: this.form.value.goal_unit!,
            goal_period: this.form.value.goal_period as GoalPeriod,
        }).subscribe(({
            next : () => { this.router.navigate(['/habits']); },
            error: (err) => {
                console.error(err);
                this.isLoading = false
                if (err.status === 409) {
                    this.nameConflict = true;
                    this.nameInput().nativeElement.focus();
                }
            },
        }));
    }

    goBack() {
        this.router.navigate(['/habits']);
    }

    get selectedPeriod() {
        return this.form.controls.goal_period.value;
    }

    setPeriod(period: string) {
        this.form.controls.goal_period.setValue(period);
    }
}