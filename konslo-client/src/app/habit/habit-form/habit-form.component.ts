import {Component, inject} from '@angular/core';
import {FormControl, FormGroup, ReactiveFormsModule, Validators} from '@angular/forms';
import {HabitService} from '../habit.service';
import {GoalPeriod} from "../habit.model";
import {Router} from "@angular/router";

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

    form = new FormGroup({
        name: new FormControl('', Validators.required),
        goal_value: new FormControl(1, [Validators.required, Validators.min(1)]),
        goal_unit: new FormControl('', Validators.required),
        goal_period: new FormControl('day', Validators.required),
    });

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
            error: () => { this.isLoading = false },
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