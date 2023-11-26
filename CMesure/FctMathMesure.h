// test de s�curit� pour �viter l'inclusion multiple des d�finitions
#ifndef _FCTMATHMESURE_H_
#define _FCTMATHMESURE_H_

#include "CMesure.h"

#ifdef _FCTMATHMESURE_EXPORTING
	#define FCTMATHMESURE_DECLSPEC __declspec(dllexport)
#else
	#define FCTMATHMESURE_DECLSPEC __declspec(dllimport)
#endif

///////////////////////////// Fonctions math�matiques //////////////////////////

// Les fonctions suivantes n�cessite l'utilisation
// de la loi de propagation des incertitudes.
// cf norme NF ENV 13005 (Guide d'Utilisation de la M�trologie : GUM)

// 1 - ce groupe de fonction est � une seule variable,
// ce qui donne, pour l'�quation y = f(x), la formule suivante:
// U�(y) = [df(x)/dx]� * U�(x)

	FCTMATHMESURE_DECLSPEC CMesure fabs (CMesure& M);
	FCTMATHMESURE_DECLSPEC CMesure sin  (CMesure& M);
	FCTMATHMESURE_DECLSPEC CMesure cos  (CMesure& M);
	FCTMATHMESURE_DECLSPEC CMesure tan  (CMesure& M);
	FCTMATHMESURE_DECLSPEC CMesure acos (CMesure& M);
	FCTMATHMESURE_DECLSPEC CMesure asin (CMesure& M);
	FCTMATHMESURE_DECLSPEC CMesure atan (CMesure& M);
	FCTMATHMESURE_DECLSPEC CMesure cosh (CMesure& M);
	FCTMATHMESURE_DECLSPEC CMesure sinh (CMesure& M);
	FCTMATHMESURE_DECLSPEC CMesure tanh (CMesure& M);
	FCTMATHMESURE_DECLSPEC CMesure log  (CMesure& M);
	FCTMATHMESURE_DECLSPEC CMesure log10(CMesure& M);
	FCTMATHMESURE_DECLSPEC CMesure exp  (CMesure& M);
	FCTMATHMESURE_DECLSPEC CMesure sqrt (CMesure& M);

	FCTMATHMESURE_DECLSPEC CMesure ceil (CMesure& M);
	FCTMATHMESURE_DECLSPEC CMesure floor(CMesure& M);


// 2 - ce groupe de fonction est � deux variables,
// ce qui donne, pour l'�quation y = f(x,y), la formule suivante:
// U�(y) = [df(x)/dx]� * U�(x) + [df(y)/dy]� * U�(y)

	FCTMATHMESURE_DECLSPEC CMesure pow  (CMesure& Base, CMesure& Puiss);
	FCTMATHMESURE_DECLSPEC CMesure pow  (double   Base, CMesure& Puiss);
	FCTMATHMESURE_DECLSPEC CMesure pow  (CMesure& Base, double   Puiss);

	FCTMATHMESURE_DECLSPEC CMesure atan2(CMesure& X, CMesure& Y);
	FCTMATHMESURE_DECLSPEC CMesure modf (CMesure& X, CMesure* Fpart);

// 3 - ce groupe de fonction me pose probl�me pour d�terminer
// comment calculer l'incertitude type. Je vais certainement faire
// un choix personnel unilat�rale parfaitement arbitraire.

	FCTMATHMESURE_DECLSPEC CMesure MIN  (CMesure& A, CMesure& B);
	FCTMATHMESURE_DECLSPEC CMesure MAX  (CMesure& A, CMesure& B);
	FCTMATHMESURE_DECLSPEC CMesure SIGN (CMesure& A);

	
#endif	// fin de _FCTMATHMESURE_H_