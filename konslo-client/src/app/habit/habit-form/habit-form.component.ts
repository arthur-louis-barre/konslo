import { Component, inject } from '@angular/core';
import {FormControl, FormGroup, ReactiveFormsModule, Validators} from '@angular/forms';
import { HabitService } from '../habit.service';
import {GoalPeriod} from "../habit.model";
import {Router} from "@angular/router";
import {MatIcon} from "@angular/material/icon";

@Component({
  selector: 'k-new-habit-form',
  imports: [ReactiveFormsModule, MatIcon],
  templateUrl: './habit-form.component.html',
  styleUrl: './habit-form.component.css'
})
export class HabitFormComponent {
  private router = inject(Router);
  private habitsService = inject(HabitService);

  form = new FormGroup({
    name: new FormControl('', Validators.required),
    goal_value: new FormControl(1, [Validators.required, Validators.min(1)]),
    goal_unit: new FormControl('', Validators.required),
    goal_period: new FormControl('day', Validators.required),
  });

  onSubmit() {
    this.habitsService.createHabit({
      name: this.form.value.name!,
      goal_value: this.form.value.goal_value!,
      goal_unit: this.form.value.goal_unit!,
      goal_period: this.form.value.goal_period as GoalPeriod,
    }).subscribe(() => {
      this.router.navigate(['/habits']);
    });
  }

  goBack() {
    this.router.navigate(['/habits']);
  }
}