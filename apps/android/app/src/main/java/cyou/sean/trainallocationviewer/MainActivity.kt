package cyou.sean.trainallocationviewer

import android.os.Bundle
import androidx.activity.ComponentActivity
import androidx.activity.compose.setContent
import androidx.activity.enableEdgeToEdge
import androidx.compose.material3.Icon
import androidx.compose.material3.Text
import androidx.compose.material3.adaptive.navigationsuite.NavigationSuiteScaffold
import androidx.compose.runtime.Composable
import androidx.compose.runtime.getValue
import androidx.compose.ui.Modifier
import androidx.compose.ui.tooling.preview.PreviewScreenSizes
import androidx.navigation.compose.currentBackStackEntryAsState
import androidx.navigation.compose.rememberNavController
import cyou.sean.trainallocationviewer.navigation.AppNavHost
import cyou.sean.trainallocationviewer.navigation.Screen
import cyou.sean.trainallocationviewer.ui.theme.TrainAllocationViewerTheme

class MainActivity : ComponentActivity() {
    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        enableEdgeToEdge()
        setContent {
            TrainAllocationViewerTheme {
                TrainAllocationViewerApp()
            }
        }
    }
}

@PreviewScreenSizes
@Composable
fun TrainAllocationViewerApp() {
    val navController = rememberNavController()
    val navBackStackEntry by navController.currentBackStackEntryAsState()
    val currentRoute = navBackStackEntry?.destination?.route

    NavigationSuiteScaffold(
        navigationSuiteItems = {
            Screen.screens.forEach { screen ->
                item(
                    icon = {
                        Icon(
                            screen.icon,
                            contentDescription = screen.label
                        )
                    },
                    label = { Text(screen.label) },
                    selected = currentRoute == screen.route,
                    onClick = {
                        navController.navigate(screen.route) {
                            // Pop up to the start destination to avoid building a large back stack
                            popUpTo(Screen.Search.route) {
                                saveState = true
                            }
                            // Avoid multiple copies of the same destination when reselecting
                            launchSingleTop = true
                            // Restore state when reselecting a previously selected item
                            restoreState = true
                        }
                    }
                )
            }
        }
    ) {
        AppNavHost(navController = navController)
    }
}

