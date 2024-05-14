import static com.kms.katalon.core.checkpoint.CheckpointFactory.findCheckpoint
import static com.kms.katalon.core.testcase.TestCaseFactory.findTestCase
import static com.kms.katalon.core.testdata.TestDataFactory.findTestData
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject

import com.kms.katalon.core.checkpoint.Checkpoint as Checkpoint
import com.kms.katalon.core.model.FailureHandling as FailureHandling
import com.kms.katalon.core.testcase.TestCase as TestCase
import com.kms.katalon.core.testdata.TestData
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.testobject.TestObject
import com.kms.katalon.core.util.KeywordUtil
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI

import groovy.json.JsonSlurper

import com.kms.katalon.core.mobile.keyword.MobileBuiltInKeywords as Mobile

import internal.GlobalVariable as GlobalVariable

import com.kms.katalon.core.annotation.BeforeTestCase
import com.kms.katalon.core.annotation.BeforeTestSuite
import com.kms.katalon.core.annotation.AfterTestCase
import com.kms.katalon.core.annotation.AfterTestSuite
import com.kms.katalon.core.context.TestCaseContext
import com.kms.katalon.core.context.TestSuiteContext

class RecuperarToken {

	private void setToken() {
		def jsonSlurper = new JsonSlurper()

		//Recuperar token SalesForce
		ResponseObject responseTokenSF = WS.sendRequest(findTestObject('Postman (1)/Token/Token'))

		WS.verifyResponseStatusCode(responseTokenSF, 200)

		def jsonResponseTokenSF = jsonSlurper.parseText(responseTokenSF.getResponseText())

		GlobalVariable.TOKEN = "Bearer "+ jsonResponseTokenSF.access_token

		print(GlobalVariable.TOKEN)
		KeywordUtil.logInfo("Token: "+ GlobalVariable.TOKEN)
		
		
	}

	@BeforeTestCase
	def beforeTestSuite() {
		//set global variables
		setToken()
	}
}

